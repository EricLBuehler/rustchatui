use rustchatui::start_ui_server;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // default port = 3000 if not given
    let port = std::env::args()
        .nth(1)
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    start_ui_server(port, "dist").await
}
