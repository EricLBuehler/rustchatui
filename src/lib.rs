use axum::Router;
use tower_http::services::ServeDir;
use anyhow::Result;
use tower_http::cors::{Any, CorsLayer};

pub async fn start_ui_server(port: u16, dist_path: impl Into<String>) -> Result<()> {
    let dist = dist_path.into();

    let cors = CorsLayer::new()
            .allow_origin(Any) // same as "*"
            .allow_methods(Any)
            .allow_headers(Any);

    // Serve the dist folder
    let app = Router::new()
        .nest_service("/", ServeDir::new(dist)).layer(cors);

    let addr = format!("localhost:{port}");
    println!("\r\n🚀 Rust Chat UI server running at http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
