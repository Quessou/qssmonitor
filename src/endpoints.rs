use axum::{extract::State, routing::get, Router};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{data::Report, Core};

pub async fn get_last_report(State(core): State<Core>) -> String {
    core.aggregator.lock().await.get_report().to_string()
}

pub async fn generate_api(core: Core) -> Router {
    Router::new()
        .route("/last_report", get(get_last_report))
        .with_state(core)
}
