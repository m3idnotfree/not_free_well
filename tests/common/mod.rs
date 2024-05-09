use not_free_well::app;
use tokio::net::TcpListener;

pub async fn spawn_app() -> std::net::SocketAddr {
    let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app()).await.unwrap();
    });
    addr
}
