#[tokio::test]
async fn ping_loopback() {
    let res = ipgeom_query::ping(
        "127.0.0.1",
        std::time::Duration::from_secs(1),
        1,
        std::time::Duration::from_secs(0),
        None,
        Some(ipgeom_query::IpVersion::V4),
    )
    .await
    .unwrap();
    assert_eq!(res.transmitted, 1);
    assert!(res.received <= 1);
}
