use super::*;

#[test]
fn xx_hash3_64_matches_the_allocating_hasher() {
    let samples: [&str; 8] = [
        "",
        "connection",
        "content-type",
        "content-length",
        "host",
        "/api/user/12345",
        "x-bench-tag",
        "a header name long enough to pass the small input cutoff",
    ];
    for sample in samples {
        let mut stack_hasher: XxHash3_64 = XxHash3_64::default();
        stack_hasher.write(sample.as_bytes());
        let stack_hash: u64 = stack_hasher.finish();

        let mut allocating_hasher: xxhash3_64::Hasher = xxhash3_64::Hasher::default();
        allocating_hasher.write(sample.as_bytes());
        let allocating_hash: u64 = allocating_hasher.finish();

        assert_eq!(
            stack_hash, allocating_hash,
            "the hash of {sample:?} must not depend on where the secret lives"
        );
    }
}

#[test]
fn xx_hash3_64_does_not_allocate() {
    let mut hasher: XxHash3_64 = XxHash3_64::default();
    hasher.write(b"connection");
    let _: u64 = hasher.finish();
    let mut reused: XxHash3_64 = XxHash3_64::default();
    reused.write(b"content-length");
    let _: u64 = reused.finish();
}

#[test]
fn xx_hash3_64_preserves_every_prefix_and_write_partition() {
    let bytes: Vec<u8> = (0..2048)
        .map(|offset: usize| (offset % 251) as u8)
        .collect();
    for length in [0, 1, 7, 16, 64, 127, 128, 129, 240, 241, 1024, 2048] {
        for width in [1, 7, 16, 63, 128, 129, 1024] {
            let mut actual: XxHash3_64 = XxHash3_64::default();
            let mut expected: xxhash3_64::Hasher = xxhash3_64::Hasher::default();
            for chunk in bytes[..length].chunks(width) {
                actual.write(chunk);
                expected.write(chunk);
                assert_eq!(actual.finish(), expected.finish());
                assert_eq!(actual.clone().finish(), expected.finish());
            }
            actual.write(&[]);
            expected.write(&[]);
            assert_eq!(actual.finish(), expected.finish());
            actual.write_u8(255);
            expected.write_u8(255);
            assert_eq!(actual.finish(), expected.finish());
        }
    }
}

#[test]
fn xx_hash3_64_hashes_a_map() {
    let mut map: HashMap<String, u32, BuildHasherDefault<XxHash3_64>> = HashMap::default();
    map.insert("connection".to_owned(), 1);
    map.insert("content-type".to_owned(), 2);
    assert_eq!(map.get("connection"), Some(&1));
    assert_eq!(map.get("content-type"), Some(&2));
    assert_eq!(map.get("missing"), None);
}
