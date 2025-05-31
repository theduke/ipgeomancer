use ipgeom_server::run;
use reqwest::Client;
use std::{
    net::{SocketAddr, TcpListener},
    path::Path,
};

#[tokio::test]
async fn api_password_hash_route_returns_json() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr: SocketAddr = listener.local_addr().unwrap();
    drop(listener);

    let server = tokio::spawn(async move {
        run(addr, Path::new(":memory:")).await.unwrap();
    });

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let client = Client::new();
    let url = format!(
        "http://{}/api/v1/password-hash-generate?method=bcrypt&password=secret",
        addr
    );
    let resp = client.get(url).send().await.unwrap();
    assert!(resp.status().is_success());
    let json: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(json["method"], "bcrypt");
    assert!(json["hash"].as_str().unwrap().starts_with("$2"));

    server.abort();
}
