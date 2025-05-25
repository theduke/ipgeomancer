#[cfg(feature = "test-online")]
use hickory_proto::rr::RecordType;

#[cfg(feature = "test-online")]
#[tokio::test]
async fn google_a_query() {
    let res = ipgeom_query::dns::authoritative_query("wasmer.io", RecordType::A, None)
        .await
        .unwrap();
    dbg!(&res);
    assert!(!res.records.is_empty());
}
