use axum::{extract::State, routing::get, Router};

use crate::{data::digest::ProductivityComputation, data::Digest, database::DatabaseAccess, Core};

pub async fn get_last_report<
    DB: DatabaseAccess + std::fmt::Debug,
    PC: ProductivityComputation, //+ std::marker::Send + std::marker::Sync,
>(
    State(core): State<Core<DB, PC>>,
) -> String {
    serde_json::to_string(&core.aggregator.lock().await.get_current_report()).unwrap()
}

pub async fn get_last_digest<DB: DatabaseAccess + std::fmt::Debug, PC: ProductivityComputation>(
    State(core): State<Core<DB, PC>>,
) -> String {
    let report = core.aggregator.lock().await.get_current_report();
    let digest = Digest::try_from(report).expect("Creating digest from report failed");
    serde_json::to_string(&digest).unwrap()
}

pub async fn generate_api<
    DB: DatabaseAccess + std::fmt::Debug + 'static,
    PC: ProductivityComputation + 'static,
>(
    core: Core<DB, PC>,
) -> Router {
    Router::new()
        .route("/last_report", get(get_last_report))
        .route("/last_digest", get(get_last_digest))
        .with_state(core)
}
