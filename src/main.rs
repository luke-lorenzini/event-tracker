use std::net::SocketAddr;

use event_tracker::app;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = app(true);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
