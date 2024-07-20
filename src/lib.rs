mod config;

use std::sync::Arc;
use axum::Router;

pub use config::AppConfig;

#[derive(Debug, Clone)]
pub(crate) struct AppState {
    pub(crate) inner: Arc<AppStateInner>,
}

#[derive(Debug, Clone)]
pub(crate) struct AppStateInner {
    pub(crate) app_config: AppConfig,
}

pub fn get_router(config: AppConfig) -> Router {
    todo!()
}