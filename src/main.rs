use esp_keepalive::{db, log, router};
use std::net::SocketAddr;
use tracing::info;


#[tokio::main]
async fn main() {
    let writer = log::init_logging();

    let db_path = "test.db";
    let database = db::Database::new(db_path)
        .await
        .expect("Failed to create database");

    db::init_tables(database.get_conn()).await.unwrap();

    let app = router::init()
        .with_state(database);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    info!("Server running on http://{}", addr);
    axum_server::bind(addr).serve(app.into_make_service()).await.unwrap();
}
