use super::*;

#[test]
fn test() {
    let headers: HashMap<_, _, BuildHasherDefault<XxHash3_64>> =
        HashMap::with_hasher(BuildHasherDefault::default());
    let data: Vec<u8> = vec![];
    let body: Cow<'_, [u8]> = Compress::from(&headers).decode(&data, 1_024_000);
    assert_eq!(*body, data);
    let _: Cow<'_, [u8]> = Compress::Gzip.encode(&[], 1_024_000);
    let _: Cow<'_, [u8]> = Compress::Deflate.encode(&[], 1_024_000);
    let _: Cow<'_, [u8]> = Compress::Br.encode(&[], 1_024_000);
    let _: Cow<'_, [u8]> = Compress::Gzip.decode(&[], 1_024_000);
    let _: Cow<'_, [u8]> = Compress::Deflate.decode(&[], 1_024_000);
    let _: Cow<'_, [u8]> = Compress::Br.decode(&[], 1_024_000);
}
