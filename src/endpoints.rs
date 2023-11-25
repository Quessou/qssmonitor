use axum::{extract::State, routing::get, Router};

use crate::{database::DatabaseAccess, Core};

pub async fn get_last_report<DB: DatabaseAccess + std::fmt::Debug>(
    State(core): State<Core<DB>>,
) -> String {
    serde_json::to_string(&core.aggregator.lock().await.get_report()).unwrap()
}

pub async fn generate_api<DB: DatabaseAccess + std::fmt::Debug + 'static>(
    core: Core<DB>,
) -> Router {
    Router::new()
        .route("/last_report", get(get_last_report))
        .with_state(core)
}
