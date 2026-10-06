use super::*;

#[test]
fn from_str_maps_known_encoding_names() {
    let gzip: Compress = "gzip".parse::<Compress>().unwrap_or_default();
    let deflate: Compress = "deflate".parse::<Compress>().unwrap_or_default();
    let brotli: Compress = "br".parse::<Compress>().unwrap_or_default();
    let unknown: Compress = "lzma".parse::<Compress>().unwrap_or_default();
    let empty: Compress = "".parse::<Compress>().unwrap_or_default();
    assert_eq!(gzip, Compress::Gzip);
    assert_eq!(deflate, Compress::Deflate);
    assert_eq!(brotli, Compress::Br);
    assert_eq!(unknown, Compress::Unknown);
    assert_eq!(empty, Compress::Unknown);
}

#[test]
fn from_str_is_case_insensitive() {
    let upper: Compress = "GZIP".parse::<Compress>().unwrap_or_default();
    let mixed: Compress = "DeFlAtE".parse::<Compress>().unwrap_or_default();
    let brotli: Compress = "BR".parse::<Compress>().unwrap_or_default();
    assert_eq!(upper, Compress::Gzip);
    assert_eq!(mixed, Compress::Deflate);
    assert_eq!(brotli, Compress::Br);
}

#[test]
fn display_renders_content_encoding_token() {
    assert_eq!(Compress::Gzip.to_string(), "gzip");
    assert_eq!(Compress::Deflate.to_string(), "deflate");
    assert_eq!(Compress::Br.to_string(), "br");
    assert_eq!(Compress::Unknown.to_string(), "");
}

#[test]
fn display_round_trips_through_from_str() {
    let variants: Vec<Compress> = vec![
        Compress::Gzip,
        Compress::Deflate,
        Compress::Br,
        Compress::Unknown,
    ];
    for variant in variants {
        let text: String = variant.to_string();
        let parsed: Compress = text.parse::<Compress>().unwrap_or_default();
        if variant == Compress::Unknown {
            assert_eq!(parsed, Compress::Unknown);
        } else {
            assert_eq!(parsed, variant);
        }
    }
}

#[test]
fn is_unknown_is_true_only_for_unknown() {
    assert!(Compress::Unknown.is_unknown());
    assert!(!Compress::Gzip.is_unknown());
    assert!(!Compress::Deflate.is_unknown());
    assert!(!Compress::Br.is_unknown());
}

#[test]
fn default_is_unknown() {
    assert_eq!(Compress::default(), Compress::Unknown);
    assert!(Compress::default().is_unknown());
}

#[test]
fn from_reads_content_encoding_header() {
    let mut headers: HashMap<String, String, BuildHasherDefault<XxHash3_64>> =
        HashMap::with_hasher(BuildHasherDefault::default());
    assert_eq!(Compress::from(&headers), Compress::Unknown);
    headers.insert(String::from("content-encoding"), String::from("gzip"));
    assert_eq!(Compress::from(&headers), Compress::Gzip);
    headers.insert(String::from("content-encoding"), String::from("deflate"));
    assert_eq!(Compress::from(&headers), Compress::Deflate);
    headers.insert(String::from("content-encoding"), String::from("br"));
    assert_eq!(Compress::from(&headers), Compress::Br);
    headers.insert(String::from("content-encoding"), String::from("lzma"));
    assert_eq!(Compress::from(&headers), Compress::Unknown);
}

#[test]
fn from_ignores_unrelated_headers() {
    let mut headers: HashMap<String, String, BuildHasherDefault<XxHash3_64>> =
        HashMap::with_hasher(BuildHasherDefault::default());
    headers.insert(String::from("accept"), String::from("gzip"));
    assert_eq!(Compress::from(&headers), Compress::Unknown);
}

#[test]
fn gzip_round_trip_restores_payload() {
    let payload: Vec<u8> = b"hyperlane gzip round trip payload".to_vec();
    let encoded: Cow<'_, [u8]> = Compress::Gzip.encode(&payload, 1_024_000);
    let decoded: Cow<'_, [u8]> = Compress::Gzip.decode(&encoded, 1_024_000);
    assert_eq!(*decoded, payload);
}

#[test]
fn deflate_round_trip_restores_payload() {
    let payload: Vec<u8> = b"hyperlane deflate round trip payload".to_vec();
    let encoded: Cow<'_, [u8]> = Compress::Deflate.encode(&payload, 1_024_000);
    let decoded: Cow<'_, [u8]> = Compress::Deflate.decode(&encoded, 1_024_000);
    assert_eq!(*decoded, payload);
}

#[test]
fn brotli_round_trip_restores_payload() {
    let payload: Vec<u8> = b"hyperlane brotli round trip payload".to_vec();
    let encoded: Cow<'_, [u8]> = Compress::Br.encode(&payload, 1_024_000);
    let decoded: Cow<'_, [u8]> = Compress::Br.decode(&encoded, 1_024_000);
    assert_eq!(*decoded, payload);
}

#[test]
fn every_known_variant_round_trips() {
    let payload: Vec<u8> = b"hyperlane all variants round trip".to_vec();
    let variants: Vec<Compress> = vec![Compress::Gzip, Compress::Deflate, Compress::Br];
    for variant in variants {
        let encoded: Cow<'_, [u8]> = variant.encode(&payload, 1_024_000);
        let decoded: Cow<'_, [u8]> = variant.decode(&encoded, 1_024_000);
        assert_eq!(*decoded, payload);
    }
}

#[test]
fn unknown_passes_data_through_unchanged() {
    let payload: Vec<u8> = b"hyperlane passthrough payload".to_vec();
    let encoded: Cow<'_, [u8]> = Compress::Unknown.encode(&payload, 1_024_000);
    let decoded: Cow<'_, [u8]> = Compress::Unknown.decode(&payload, 1_024_000);
    assert_eq!(*encoded, payload);
    assert_eq!(*decoded, payload);
}

#[test]
fn empty_input_round_trips_for_every_variant() {
    let payload: Vec<u8> = Vec::new();
    let variants: Vec<Compress> = vec![
        Compress::Gzip,
        Compress::Deflate,
        Compress::Br,
        Compress::Unknown,
    ];
    for variant in variants {
        let encoded: Cow<'_, [u8]> = variant.encode(&payload, 1_024_000);
        let decoded: Cow<'_, [u8]> = variant.decode(&encoded, 1_024_000);
        assert_eq!(*decoded, payload);
    }
}

#[test]
fn corrupt_input_decodes_to_empty_for_known_variants() {
    let garbage: Vec<u8> = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05];
    let gzip: Cow<'_, [u8]> = Compress::Gzip.decode(&garbage, 1_024_000);
    let deflate: Cow<'_, [u8]> = Compress::Deflate.decode(&garbage, 1_024_000);
    let brotli: Cow<'_, [u8]> = Compress::Br.decode(&garbage, 1_024_000);
    assert!(gzip.is_empty());
    assert!(deflate.is_empty());
    assert!(brotli.is_empty());
}

#[test]
fn large_payload_round_trips_across_all_variants() {
    let mut payload: Vec<u8> = Vec::with_capacity(65_536);
    for index in 0..65_536u32 {
        payload.push((index % 251) as u8);
    }
    let variants: Vec<Compress> = vec![Compress::Gzip, Compress::Deflate, Compress::Br];
    for variant in variants {
        let encoded: Cow<'_, [u8]> = variant.encode(&payload, 1_024_000);
        let decoded: Cow<'_, [u8]> = variant.decode(&encoded, 1_024_000);
        assert_eq!(decoded.len(), payload.len());
        assert_eq!(*decoded, payload);
    }
}

#[test]
fn variants_are_ordered_and_hashable() {
    assert!(Compress::Gzip < Compress::Deflate);
    assert!(Compress::Deflate < Compress::Br);
    assert!(Compress::Br < Compress::Unknown);
    let mut set: HashSet<Compress> = HashSet::new();
    set.insert(Compress::Gzip);
    set.insert(Compress::Gzip);
    set.insert(Compress::Br);
    assert_eq!(set.len(), 2);
}
