#[cfg(feature = "test-online")]
#[tokio::test]
async fn domain_whois_query() {
    let _res = ipgeom_query::domain_whois("example.com").await.unwrap();
}
