#[cfg(feature = "test-online")]
use ipgeom_server::run;
#[cfg(feature = "test-online")]
use reqwest::Client;
#[cfg(feature = "test-online")]
use std::{
    net::{SocketAddr, TcpListener},
    path::Path,
};

#[cfg(feature = "test-online")]
#[tokio::test]
async fn api_dns_route_returns_json() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr: SocketAddr = listener.local_addr().unwrap();
    drop(listener);

    let server = tokio::spawn(async move {
        run(addr, Path::new(":memory:")).await.unwrap();
    });

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let client = Client::new();
    let url = format!(
        "http://{}/api/v1/query/dns?name=example.com&record_type=A",
        addr
    );
    let resp = client.get(url).send().await.unwrap();
    assert!(resp.status().is_success());
    let _json: serde_json::Value = resp.json().await.unwrap();

    server.abort();
}
