use std::net::SocketAddr;

use event_tracker::app;
use log::info;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    info!("starting up");
    let app = app(true);
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
