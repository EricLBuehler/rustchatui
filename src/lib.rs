use anyhow::Result;
use axum::{Router, response::IntoResponse};
use include_dir::{Dir, include_dir};
use tower_http::cors::{Any, CorsLayer};

static DIST_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/dist");

#[derive(serde::Serialize)]
#[allow(non_snake_case)]
struct AppConfig {
    serverUrl: String,
    apiKey: String,
}

pub async fn start_ui_server(
    ui_port: u16,
    api_port: Option<u16>,
    server_url: Option<String>,
    api_key: Option<String>,
) -> Result<()> {
    // CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let api_url = if api_port.is_some() {
        format!("http://localhost:{}/v1/", api_port.unwrap())
    } else {
        server_url.unwrap()
    };

    let api_key = if api_key.is_some() {
        api_key.unwrap()
    } else {
        "".to_string()
    };

    // Custom handler to serve files from DIST_DIR
    let app = Router::new()
        .route(
            "/app-config.json", // Capture the API port to return in the config
            axum::routing::get(move || async move {
                axum::Json(AppConfig {
                    serverUrl: api_url,
                    apiKey: api_key,
                })
            }),
        )
        .fallback(axum::routing::get(serve_file))
        .layer(cors);

    let addr = format!("0.0.0.0:{ui_port}");
    println!("🖥️ Rust Chat UI server running at http://localhost:{ui_port}");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn serve_file(req: axum::http::Request<axum::body::Body>) -> impl IntoResponse {
    let path = req.uri().path().trim_start_matches('/');

    let file = if path.is_empty() {
        DIST_DIR.get_file("index.html")
    } else {
        DIST_DIR.get_file(path)
    };

    match file {
        Some(f) => {
            let mime = mime_guess::from_path(f.path()).first_or_octet_stream();
            (
                [(axum::http::header::CONTENT_TYPE, mime.as_ref())],
                f.contents(),
            )
                .into_response()
        }
        None => (axum::http::StatusCode::NOT_FOUND, "Not Found").into_response(),
    }
}
