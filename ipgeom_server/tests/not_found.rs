use std::{
    net::{SocketAddr, TcpListener},
    path::Path,
};

use ipgeom_server::run;
use reqwest::Client;

#[tokio::test]
async fn unknown_route_returns_html_404() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr: SocketAddr = listener.local_addr().unwrap();
    drop(listener);

    let server = tokio::spawn(async move {
        run(addr, Path::new(":memory:")).await.unwrap();
    });

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let client = Client::new();
    let url = format!("http://{}/does-not-exist", addr);
    let resp = client.get(url).send().await.unwrap();
    assert_eq!(resp.status(), reqwest::StatusCode::NOT_FOUND);
    let body = resp.text().await.unwrap();
    assert!(body.contains("Not Found"));

    server.abort();
}

#[tokio::test]
async fn unknown_api_route_returns_json() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr: SocketAddr = listener.local_addr().unwrap();
    drop(listener);

    let server = tokio::spawn(async move {
        run(addr, Path::new(":memory:")).await.unwrap();
    });

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let client = Client::new();
    let url = format!("http://{}/api/v1/does-not-exist", addr);
    let resp = client.get(url).send().await.unwrap();
    assert_eq!(resp.status(), reqwest::StatusCode::NOT_FOUND);
    let json: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(json["error"], "Not Found");

    server.abort();
}
