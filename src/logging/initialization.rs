use tracing::subscriber::DefaultGuard;
use tracing::Level;
use tracing_subscriber;

pub fn initialize_subscriber() -> DefaultGuard {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_default(subscriber)
}
