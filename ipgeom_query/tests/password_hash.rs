#[test]
fn bcrypt_hash_roundtrip() {
    let pw = "secret";
    let hash = ipgeom_query::generate_bcrypt_hash(pw).unwrap();
    assert!(bcrypt::verify(pw, &hash).unwrap());
}
