use anyhow::Result;
use clap::Parser;
use rustchatui::start_ui_server;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Port for this UI web server
    #[arg(long, default_value_t = 8001)]
    pub ui_port: u16,

    /// Port for Local API server (Axum backend)
    #[arg(long, default_value = None)]
    pub api_port: Option<u16>,

    /// Remote API server url: http://api.openai.com/v1
    #[arg(long, default_value = None)]
    pub server_url: Option<String>,

    /// api key for local or remote API server
    #[arg(long, default_value = None)]
    pub api_key: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    assert!(
        args.api_port.is_some() || args.server_url.is_some(),
        "Need to provide at least one of them: local `api-port` or remote API `server-url`"
    );
    if args.server_url.is_some() {
        assert!(
            args.api_key.is_some(),
            "API key must provided for remote API server"
        );
    }
    start_ui_server(args.ui_port, args.api_port, args.server_url, args.api_key).await
}
