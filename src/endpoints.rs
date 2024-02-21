use axum::{extract::State, routing::get, Router};

use crate::{data::digest::ProductivityComputation, database::DatabaseAccess, Core};

pub async fn get_last_report<
    DB: DatabaseAccess + std::fmt::Debug + std::marker::Sync + 'static,
    PC: ProductivityComputation + 'static,
>(
    State(core): State<Core<DB, PC>>,
) -> String {
    let last_report = core.get_last_report().await;
    serde_json::to_string(&last_report).unwrap()
}

pub async fn get_last_digest<
    DB: DatabaseAccess + std::fmt::Debug + std::marker::Sync + 'static,
    PC: ProductivityComputation + 'static,
>(
    State(core): State<Core<DB, PC>>,
) -> String {
    let report = core.get_last_report().await;
    // TODO(mmiko) : Remove explicit reference to digest_builder here
    let digest = core.digest_builder.lock().await.build_digest(report);

    serde_json::to_string(&digest).unwrap()
}

pub async fn generate_api<
    DB: DatabaseAccess + std::fmt::Debug + std::marker::Sync + 'static,
    PC: ProductivityComputation + 'static,
>(
    core: Core<DB, PC>,
) -> Router {
    Router::new()
        .route("/last_report", get(get_last_report))
        .route("/last_digest", get(get_last_digest))
        .with_state(core)
}
