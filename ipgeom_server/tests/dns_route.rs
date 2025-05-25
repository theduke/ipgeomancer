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
async fn dns_route_no_server_param() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr: SocketAddr = listener.local_addr().unwrap();
    drop(listener);

    let server = tokio::spawn(async move {
        run(addr, Path::new(":memory:")).await.unwrap();
    });

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let client = Client::new();
    let url = format!("http://{}/dns?name=google.com&record_type=A", addr);
    let resp = client.get(url).send().await.unwrap();
    assert!(resp.status().is_success());

    server.abort();
}
