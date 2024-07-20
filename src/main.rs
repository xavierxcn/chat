use anyhow::{anyhow, Result};
use chat::{AppConfig, get_router};
use tracing::{info, level_filters::LevelFilter, warn};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt,Layer as _};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let app_config = AppConfig::load()?;
    let port = app_config.server.port;
    let addr = format!("0.0.0.0:{}", port);
    info!("Starting server on {}", addr);
    // Start the server and handle incoming connections
    let listener = TcpListener::bind(addr).await?;
    let app = get_router(app_config);

    axum::serve(listener, app.into_make_service()).await?;


    Ok(())
}
