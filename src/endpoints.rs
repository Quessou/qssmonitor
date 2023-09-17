use axum::{extract::State, routing::get, Router};




use crate::{Core};

pub async fn get_last_report(State(core): State<Core>) -> String {
    serde_json::to_string(&core.aggregator.lock().await.get_report()).unwrap()
}

pub async fn generate_api(core: Core) -> Router {
    Router::new()
        .route("/last_report", get(get_last_report))
        .with_state(core)
}
