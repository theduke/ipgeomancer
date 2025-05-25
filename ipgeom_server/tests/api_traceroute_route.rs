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
fn is_root() -> bool {
    #[cfg(unix)]
    {
        unsafe { libc::geteuid() == 0 }
    }
    #[cfg(not(unix))]
    {
        true
    }
}

#[cfg(feature = "test-online")]
#[tokio::test]
async fn api_traceroute_route_returns_json() {
    if !is_root() {
        eprintln!("skipping traceroute test: not running as root");
        return;
    }
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr: SocketAddr = listener.local_addr().unwrap();
    drop(listener);

    let server = tokio::spawn(async move {
        run(addr, Path::new(":memory:")).await.unwrap();
    });

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let client = Client::new();
    let url = format!(
        "http://{}/api/v1/query/traceroute?host=127.0.0.1&max_hops=1&queries=1",
        addr
    );
    let resp = client.get(url).send().await.unwrap();
    assert!(resp.status().is_success());
    let _json: serde_json::Value = resp.json().await.unwrap();

    server.abort();
}
