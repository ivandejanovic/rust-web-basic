mod router;
mod service;

use log::info;

use self::router::create_router;


#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let app = create_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9000").await.unwrap();

    info!("starting server");
    axum::serve(listener, app).await.unwrap();
}
