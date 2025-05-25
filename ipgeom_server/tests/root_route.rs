use std::{
    net::{SocketAddr, TcpListener},
    path::Path,
};

use ipgeom_server::run;
use reqwest::Client;

#[tokio::test]
async fn myip_returns_client_ip() {
    // pick a random available port
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr: SocketAddr = listener.local_addr().unwrap();
    drop(listener);

    let server = tokio::spawn(async move {
        run(addr, Path::new(":memory:")).await.unwrap();
    });

    // give the server a moment to start
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let client = Client::new();
    let url = format!("http://{}/myip", addr);
    let resp = client.get(url).send().await.unwrap();
    assert!(resp.status().is_success());
    let body = resp.text().await.unwrap();
    assert!(body.contains("127.0.0.1"));

    server.abort();
}
