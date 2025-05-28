use std::{
    net::{SocketAddr, TcpListener},
    path::Path,
};

use ipgeom_server::run;
use reqwest::Client;

#[tokio::test]
async fn web_pages_without_query_show_no_error() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr: SocketAddr = listener.local_addr().unwrap();
    drop(listener);

    let server = tokio::spawn(async move {
        run(addr, Path::new(":memory:")).await.unwrap();
    });

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let client = Client::new();
    let paths = ["/ping", "/traceroute", "/dns", "/whois", "/rdap", "/cert"];
    for path in paths {
        let url = format!("http://{}{}", addr, path);
        let resp = client.get(&url).send().await.unwrap();
        assert!(resp.status().is_success(), "{}", path);
        let body = resp.text().await.unwrap();
        assert!(
            !body.contains("notification is-danger"),
            "error shown for {}",
            path
        );
    }

    server.abort();
}
