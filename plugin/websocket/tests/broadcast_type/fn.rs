use super::*;

#[test]
fn point_to_point_key_is_independent_of_identifier_order() {
    let ascending: String = BroadcastType::<String>::get_key(BroadcastType::PointToPoint(
        String::from("alice"),
        String::from("bob"),
    ));
    let descending: String = BroadcastType::<String>::get_key(BroadcastType::PointToPoint(
        String::from("bob"),
        String::from("alice"),
    ));
    assert_eq!(ascending, descending);
    assert_eq!(ascending, String::from("ptp--alice-bob"));
}

#[test]
fn point_to_point_key_repeats_a_self_paired_identifier() {
    let key: String = BroadcastType::<String>::get_key(BroadcastType::PointToPoint(
        String::from("carol"),
        String::from("carol"),
    ));
    assert_eq!(key, String::from("ptp--carol-carol"));
}

#[test]
fn point_to_point_key_orders_numeric_identifiers_by_value() {
    let key: String = BroadcastType::<u8>::get_key(BroadcastType::PointToPoint(10, 2));
    assert_eq!(key, String::from("ptp--2-10"));
    let swapped: String = BroadcastType::<u8>::get_key(BroadcastType::PointToPoint(2, 10));
    assert_eq!(key, swapped);
}

#[test]
fn point_to_point_key_orders_floating_point_identifiers_by_value() {
    let key: String = BroadcastType::<f64>::get_key(BroadcastType::PointToPoint(1.5, 0.5));
    assert_eq!(key, String::from("ptp--0.5-1.5"));
    let swapped: String = BroadcastType::<f64>::get_key(BroadcastType::PointToPoint(0.5, 1.5));
    assert_eq!(key, swapped);
}

#[test]
fn point_to_group_key_is_the_group_name_behind_the_point_to_group_prefix() {
    let key: String =
        BroadcastType::<String>::get_key(BroadcastType::PointToGroup(String::from("room-7")));
    assert_eq!(key, String::from("ptg--room-7"));
}

#[test]
fn unknown_broadcast_type_key_is_the_empty_string() {
    let key: String = BroadcastType::<String>::get_key(BroadcastType::Unknown);
    assert_eq!(key, String::new());
    assert!(key.is_empty());
}

#[test]
fn get_key_renders_scalar_payloads_through_their_display_form() {
    let character: String = BroadcastType::<char>::get_key(BroadcastType::PointToGroup('x'));
    assert_eq!(character, String::from("ptg--x"));
    let boolean: String = BroadcastType::<bool>::get_key(BroadcastType::PointToPoint(true, false));
    assert_eq!(boolean, String::from("ptp--false-true"));
    let slice: String = BroadcastType::<&str>::get_key(BroadcastType::PointToGroup("room"));
    assert_eq!(slice, String::from("ptg--room"));
    let signed: String = BroadcastType::<i64>::get_key(BroadcastType::PointToPoint(-3, 4));
    assert_eq!(signed, String::from("ptp---3-4"));
    let unsigned: String = BroadcastType::<u128>::get_key(BroadcastType::PointToGroup(
        340282366920938463463374607431768211455,
    ));
    assert_eq!(
        unsigned,
        String::from("ptg--340282366920938463463374607431768211455")
    );
}

#[test]
fn get_key_renders_network_payloads_through_their_display_form() {
    let v4: String =
        BroadcastType::<Ipv4Addr>::get_key(BroadcastType::PointToGroup(Ipv4Addr::new(10, 0, 0, 1)));
    assert_eq!(v4, String::from("ptg--10.0.0.1"));
    let v6: String =
        BroadcastType::<Ipv6Addr>::get_key(BroadcastType::PointToGroup(Ipv6Addr::LOCALHOST));
    assert_eq!(v6, String::from("ptg--::1"));
    let ip: String = BroadcastType::<IpAddr>::get_key(BroadcastType::PointToGroup(IpAddr::V4(
        Ipv4Addr::new(127, 0, 0, 1),
    )));
    assert_eq!(ip, String::from("ptg--127.0.0.1"));
    let socket: String = BroadcastType::<SocketAddr>::get_key(BroadcastType::PointToGroup(
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
    ));
    assert_eq!(socket, String::from("ptg--127.0.0.1:8080"));
}

#[test]
fn get_key_renders_non_zero_payloads_through_their_display_form() {
    let nine: NonZeroU8 = NonZeroU8::new(9).unwrap_or(NonZeroU8::MIN);
    let unsigned: String = BroadcastType::<NonZeroU8>::get_key(BroadcastType::PointToGroup(nine));
    assert_eq!(unsigned, String::from("ptg--9"));
    let minus_two: NonZeroI32 = NonZeroI32::new(-2).unwrap_or(NonZeroI32::MIN);
    let signed: String =
        BroadcastType::<NonZeroI32>::get_key(BroadcastType::PointToGroup(minus_two));
    assert_eq!(signed, String::from("ptg---2"));
    let size: NonZeroUsize = NonZeroUsize::new(1024).unwrap_or(NonZeroUsize::MIN);
    let on_host: String = BroadcastType::<NonZeroUsize>::get_key(BroadcastType::PointToGroup(size));
    assert_eq!(on_host, String::from("ptg--1024"));
}

#[test]
fn get_key_never_merges_a_point_to_point_key_with_a_point_to_group_key() {
    let group: String =
        BroadcastType::<String>::get_key(BroadcastType::PointToGroup(String::from("a-b")));
    let pair: String = BroadcastType::<String>::get_key(BroadcastType::PointToPoint(
        String::from("a"),
        String::from("b"),
    ));
    assert_ne!(group, pair);
    let unknown: String = BroadcastType::<String>::get_key(BroadcastType::Unknown);
    assert_ne!(group, unknown);
    assert_ne!(pair, unknown);
}

#[test]
fn default_broadcast_type_is_unknown() {
    let owned: BroadcastType<String> = BroadcastType::default();
    assert_eq!(owned, BroadcastType::Unknown);
    let borrowed: BroadcastType<&str> = BroadcastType::default();
    assert_eq!(borrowed, BroadcastType::Unknown);
    let numeric: BroadcastType<u8> = BroadcastType::default();
    assert!(matches!(numeric, BroadcastType::Unknown));
}

#[test]
fn broadcast_type_is_copy_when_its_payload_is_copy() {
    let original: BroadcastType<u8> = BroadcastType::PointToPoint(3, 4);
    let moved: BroadcastType<u8> = original;
    let reused: BroadcastType<u8> = original;
    assert_eq!(moved, reused);
    assert_eq!(original, BroadcastType::PointToPoint(3, 4));
}

#[test]
fn broadcast_type_with_an_owned_payload_survives_being_cloned() {
    let original: BroadcastType<String> = BroadcastType::PointToGroup(String::from("room"));
    let clone: BroadcastType<String> = original.clone();
    assert_eq!(original, clone);
    assert!(matches!(original, BroadcastType::PointToGroup(_)));
}

#[test]
fn distinct_broadcast_types_collapse_in_a_hash_set_by_value() {
    let mut set: HashSet<BroadcastType<String>> = HashSet::new();
    set.insert(BroadcastType::PointToGroup(String::from("room")));
    set.insert(BroadcastType::PointToGroup(String::from("room")));
    set.insert(BroadcastType::PointToPoint(
        String::from("a"),
        String::from("b"),
    ));
    set.insert(BroadcastType::PointToPoint(
        String::from("b"),
        String::from("a"),
    ));
    set.insert(BroadcastType::Unknown);
    set.insert(BroadcastType::Unknown);
    assert_eq!(set.len(), 4);
    assert!(set.contains(&BroadcastType::PointToPoint(
        String::from("a"),
        String::from("b")
    )));
    assert!(set.contains(&BroadcastType::PointToPoint(
        String::from("b"),
        String::from("a")
    )));
}

#[test]
fn point_to_point_equality_ignores_the_sorted_key_but_not_the_tuple_order() {
    let forward: BroadcastType<String> =
        BroadcastType::PointToPoint(String::from("a"), String::from("b"));
    let reverse: BroadcastType<String> =
        BroadcastType::PointToPoint(String::from("b"), String::from("a"));
    assert_ne!(forward, reverse);
    assert_eq!(
        BroadcastType::get_key(forward.clone()),
        BroadcastType::get_key(reverse)
    );
    assert_eq!(forward, forward.clone());
    assert_ne!(forward, BroadcastType::Unknown);
}

#[test]
fn debug_names_the_variant_and_renders_its_payload() {
    let group: String = format!(
        "{:?}",
        BroadcastType::<String>::PointToGroup(String::from("room"))
    );
    assert_eq!(group, String::from("PointToGroup(\"room\")"));
    let pair: String = format!("{:?}", BroadcastType::<&str>::PointToPoint("a", "b"));
    assert_eq!(pair, String::from("PointToPoint(\"a\", \"b\")"));
    let unknown: String = format!("{:?}", BroadcastType::<u8>::Unknown);
    assert_eq!(unknown, String::from("Unknown"));
}

#[test]
fn broadcast_type_trait_covers_owned_scalar_identifiers() {
    let _: BroadcastType<String> = BroadcastType::Unknown;
    let _: BroadcastType<&str> = BroadcastType::Unknown;
    let _: BroadcastType<char> = BroadcastType::Unknown;
    let _: BroadcastType<bool> = BroadcastType::Unknown;
    let _: BroadcastType<i8> = BroadcastType::Unknown;
    let _: BroadcastType<i16> = BroadcastType::Unknown;
    let _: BroadcastType<i32> = BroadcastType::Unknown;
    let _: BroadcastType<i64> = BroadcastType::Unknown;
    let _: BroadcastType<i128> = BroadcastType::Unknown;
    let _: BroadcastType<isize> = BroadcastType::Unknown;
    let _: BroadcastType<u8> = BroadcastType::Unknown;
    let _: BroadcastType<u16> = BroadcastType::Unknown;
    let _: BroadcastType<u32> = BroadcastType::Unknown;
    let _: BroadcastType<u64> = BroadcastType::Unknown;
    let _: BroadcastType<u128> = BroadcastType::Unknown;
    let _: BroadcastType<usize> = BroadcastType::Unknown;
    let _: BroadcastType<f32> = BroadcastType::Unknown;
    let _: BroadcastType<f64> = BroadcastType::Unknown;
}

#[test]
fn broadcast_type_trait_covers_network_non_zero_and_infallible_identifiers() {
    let _: BroadcastType<IpAddr> = BroadcastType::Unknown;
    let _: BroadcastType<Ipv4Addr> = BroadcastType::Unknown;
    let _: BroadcastType<Ipv6Addr> = BroadcastType::Unknown;
    let _: BroadcastType<SocketAddr> = BroadcastType::Unknown;
    let _: BroadcastType<NonZeroU8> = BroadcastType::Unknown;
    let _: BroadcastType<NonZeroU16> = BroadcastType::Unknown;
    let _: BroadcastType<NonZeroU32> = BroadcastType::Unknown;
    let _: BroadcastType<NonZeroU64> = BroadcastType::Unknown;
    let _: BroadcastType<NonZeroU128> = BroadcastType::Unknown;
    let _: BroadcastType<NonZeroUsize> = BroadcastType::Unknown;
    let _: BroadcastType<NonZeroI8> = BroadcastType::Unknown;
    let _: BroadcastType<NonZeroI16> = BroadcastType::Unknown;
    let _: BroadcastType<NonZeroI32> = BroadcastType::Unknown;
    let _: BroadcastType<NonZeroI64> = BroadcastType::Unknown;
    let _: BroadcastType<NonZeroI128> = BroadcastType::Unknown;
    let _: BroadcastType<NonZeroIsize> = BroadcastType::Unknown;
    let _: BroadcastType<Infallible> = BroadcastType::Unknown;
}

#[test]
fn broadcast_type_trait_covers_reference_identifiers() {
    let _: BroadcastType<&String> = BroadcastType::Unknown;
    let _: BroadcastType<&&str> = BroadcastType::Unknown;
    let _: BroadcastType<&char> = BroadcastType::Unknown;
    let _: BroadcastType<&bool> = BroadcastType::Unknown;
    let _: BroadcastType<&i8> = BroadcastType::Unknown;
    let _: BroadcastType<&i16> = BroadcastType::Unknown;
    let _: BroadcastType<&i32> = BroadcastType::Unknown;
    let _: BroadcastType<&i64> = BroadcastType::Unknown;
    let _: BroadcastType<&i128> = BroadcastType::Unknown;
    let _: BroadcastType<&isize> = BroadcastType::Unknown;
    let _: BroadcastType<&u8> = BroadcastType::Unknown;
    let _: BroadcastType<&u16> = BroadcastType::Unknown;
    let _: BroadcastType<&u32> = BroadcastType::Unknown;
    let _: BroadcastType<&u128> = BroadcastType::Unknown;
    let _: BroadcastType<&usize> = BroadcastType::Unknown;
    let _: BroadcastType<&f32> = BroadcastType::Unknown;
    let _: BroadcastType<&f64> = BroadcastType::Unknown;
    let _: BroadcastType<&IpAddr> = BroadcastType::Unknown;
    let _: BroadcastType<&Ipv4Addr> = BroadcastType::Unknown;
    let _: BroadcastType<&Ipv6Addr> = BroadcastType::Unknown;
    let _: BroadcastType<&SocketAddr> = BroadcastType::Unknown;
    let _: BroadcastType<&NonZeroU8> = BroadcastType::Unknown;
    let _: BroadcastType<&NonZeroU16> = BroadcastType::Unknown;
    let _: BroadcastType<&NonZeroU32> = BroadcastType::Unknown;
    let _: BroadcastType<&NonZeroU64> = BroadcastType::Unknown;
    let _: BroadcastType<&NonZeroU128> = BroadcastType::Unknown;
    let _: BroadcastType<&NonZeroUsize> = BroadcastType::Unknown;
    let _: BroadcastType<&NonZeroI8> = BroadcastType::Unknown;
    let _: BroadcastType<&NonZeroI16> = BroadcastType::Unknown;
    let _: BroadcastType<&NonZeroI32> = BroadcastType::Unknown;
    let _: BroadcastType<&NonZeroI64> = BroadcastType::Unknown;
    let _: BroadcastType<&NonZeroI128> = BroadcastType::Unknown;
    let _: BroadcastType<&NonZeroIsize> = BroadcastType::Unknown;
    let _: BroadcastType<&Infallible> = BroadcastType::Unknown;
}
