use super::*;

#[test]
fn a_new_web_socket_exposes_an_empty_broadcast_map() {
    let created: WebSocket = WebSocket::new();
    let defaulted: WebSocket = WebSocket::default();
    let created_missing: Option<ReceiverCount> =
        created.get_broadcast_map().receiver_count("absent");
    let defaulted_missing: Option<ReceiverCount> =
        defaulted.get_broadcast_map().receiver_count("absent");
    assert_eq!(created_missing, None);
    assert_eq!(defaulted_missing, None);
}

#[test]
fn receiver_count_is_zero_before_anything_subscribes() {
    let web_socket: WebSocket = WebSocket::new();
    let group: ReceiverCount =
        web_socket.receiver_count(BroadcastType::<String>::PointToGroup(String::from("room")));
    let pair: ReceiverCount = web_socket.receiver_count(BroadcastType::<String>::PointToPoint(
        String::from("a"),
        String::from("b"),
    ));
    let unknown: ReceiverCount = web_socket.receiver_count(BroadcastType::<String>::Unknown);
    assert_eq!(group, 0);
    assert_eq!(pair, 0);
    assert_eq!(unknown, 0);
}

#[test]
fn receiver_count_tracks_every_subscriber_of_the_generated_key() {
    let web_socket: WebSocket = WebSocket::new();
    let broadcast_type: BroadcastType<String> = BroadcastType::PointToGroup(String::from("room"));
    let key: String = BroadcastType::get_key(broadcast_type.clone());
    let _first: BroadcastMapReceiver<Vec<u8>> = web_socket
        .get_broadcast_map()
        .subscribe_or_insert(&key, DEFAULT_BROADCAST_SENDER_CAPACITY);
    let _second: BroadcastMapReceiver<Vec<u8>> = web_socket
        .get_broadcast_map()
        .subscribe_or_insert(&key, DEFAULT_BROADCAST_SENDER_CAPACITY);
    let count: ReceiverCount = web_socket.receiver_count(broadcast_type);
    assert_eq!(count, 2);
}

#[test]
fn both_point_to_point_orders_reach_the_same_channel() {
    let web_socket: WebSocket = WebSocket::new();
    let forward: BroadcastType<String> =
        BroadcastType::PointToPoint(String::from("alice"), String::from("bob"));
    let reverse: BroadcastType<String> =
        BroadcastType::PointToPoint(String::from("bob"), String::from("alice"));
    let key: String = BroadcastType::get_key(forward.clone());
    let _receiver: BroadcastMapReceiver<Vec<u8>> = web_socket
        .get_broadcast_map()
        .subscribe_or_insert(&key, DEFAULT_BROADCAST_SENDER_CAPACITY);
    let forward_count: ReceiverCount = web_socket.receiver_count(forward);
    let reverse_count: ReceiverCount = web_socket.receiver_count(reverse);
    assert_eq!(key, String::from("ptp--alice-bob"));
    assert_eq!(forward_count, 1);
    assert_eq!(reverse_count, 1);
}

#[test]
fn receiver_count_before_connected_reports_one_more_than_the_live_count() {
    let web_socket: WebSocket = WebSocket::new();
    let broadcast_type: BroadcastType<String> = BroadcastType::PointToGroup(String::from("room"));
    let key: String = BroadcastType::get_key(broadcast_type.clone());
    let before_any: ReceiverCount =
        web_socket.receiver_count_before_connected(broadcast_type.clone());
    let _first: BroadcastMapReceiver<Vec<u8>> = web_socket
        .get_broadcast_map()
        .subscribe_or_insert(&key, DEFAULT_BROADCAST_SENDER_CAPACITY);
    let after_one: ReceiverCount =
        web_socket.receiver_count_before_connected(broadcast_type.clone());
    let _second: BroadcastMapReceiver<Vec<u8>> = web_socket
        .get_broadcast_map()
        .subscribe_or_insert(&key, DEFAULT_BROADCAST_SENDER_CAPACITY);
    let _third: BroadcastMapReceiver<Vec<u8>> = web_socket
        .get_broadcast_map()
        .subscribe_or_insert(&key, DEFAULT_BROADCAST_SENDER_CAPACITY);
    let after_three: ReceiverCount =
        web_socket.receiver_count_before_connected(broadcast_type.clone());
    let untouched: ReceiverCount =
        web_socket
            .receiver_count_before_connected(BroadcastType::<String>::PointToGroup(String::new()));
    assert_eq!(before_any, 1);
    assert_eq!(after_one, 2);
    assert_eq!(after_three, 4);
    assert_eq!(untouched, 1);
}

#[test]
fn receiver_count_after_closed_never_drops_below_zero() {
    let web_socket: WebSocket = WebSocket::new();
    let broadcast_type: BroadcastType<String> = BroadcastType::PointToGroup(String::from("room"));
    let key: String = BroadcastType::get_key(broadcast_type.clone());
    let with_no_subscriber: ReceiverCount =
        web_socket.receiver_count_after_closed(broadcast_type.clone());
    let _first: BroadcastMapReceiver<Vec<u8>> = web_socket
        .get_broadcast_map()
        .subscribe_or_insert(&key, DEFAULT_BROADCAST_SENDER_CAPACITY);
    let with_one_subscriber: ReceiverCount =
        web_socket.receiver_count_after_closed(broadcast_type.clone());
    let _second: BroadcastMapReceiver<Vec<u8>> = web_socket
        .get_broadcast_map()
        .subscribe_or_insert(&key, DEFAULT_BROADCAST_SENDER_CAPACITY);
    let with_two_subscribers: ReceiverCount =
        web_socket.receiver_count_after_closed(broadcast_type);
    let untouched: ReceiverCount = web_socket
        .receiver_count_after_closed(BroadcastType::<String>::PointToGroup(String::new()));
    assert_eq!(with_no_subscriber, 0);
    assert_eq!(with_one_subscriber, 0);
    assert_eq!(with_two_subscribers, 1);
    assert_eq!(untouched, 0);
}

#[test]
fn try_send_reports_no_channel_for_an_unused_broadcast_type() {
    let web_socket: WebSocket = WebSocket::new();
    let result: Result<Option<ReceiverCount>, BroadcastMapSendError<Vec<u8>>> = web_socket
        .try_send(
            BroadcastType::<String>::PointToGroup(String::from("room")),
            "payload",
        );
    let count: Option<ReceiverCount> = result.unwrap();
    assert_eq!(count, None);
}

#[tokio::test]
async fn try_send_delivers_the_payload_to_every_subscriber() {
    let web_socket: WebSocket = WebSocket::new();
    let broadcast_type: BroadcastType<String> = BroadcastType::PointToGroup(String::from("room"));
    let key: String = BroadcastType::get_key(broadcast_type.clone());
    let mut first: BroadcastMapReceiver<Vec<u8>> = web_socket
        .get_broadcast_map()
        .subscribe_or_insert(&key, DEFAULT_BROADCAST_SENDER_CAPACITY);
    let mut second: BroadcastMapReceiver<Vec<u8>> = web_socket
        .get_broadcast_map()
        .subscribe_or_insert(&key, DEFAULT_BROADCAST_SENDER_CAPACITY);
    let result: Result<Option<ReceiverCount>, BroadcastMapSendError<Vec<u8>>> =
        web_socket.try_send(broadcast_type, String::from("hello"));
    let count: Option<ReceiverCount> = result.unwrap();
    assert_eq!(count, Some(2));
    let first_message: Result<Vec<u8>, RecvError> = first.recv().await;
    let second_message: Result<Vec<u8>, RecvError> = second.recv().await;
    assert_eq!(first_message.unwrap(), String::from("hello").into_bytes());
    assert_eq!(second_message.unwrap(), String::from("hello").into_bytes());
}

#[tokio::test]
async fn try_send_accepts_every_byte_vector_payload_shape() {
    let web_socket: WebSocket = WebSocket::new();
    let broadcast_type: BroadcastType<String> = BroadcastType::PointToGroup(String::from("room"));
    let key: String = BroadcastType::get_key(broadcast_type.clone());
    let mut receiver: BroadcastMapReceiver<Vec<u8>> = web_socket
        .get_broadcast_map()
        .subscribe_or_insert(&key, DEFAULT_BROADCAST_SENDER_CAPACITY);
    let from_str: Result<Option<ReceiverCount>, BroadcastMapSendError<Vec<u8>>> =
        web_socket.try_send(broadcast_type.clone(), "one");
    let from_string: Result<Option<ReceiverCount>, BroadcastMapSendError<Vec<u8>>> =
        web_socket.try_send(broadcast_type.clone(), String::from("two"));
    let from_bytes: Result<Option<ReceiverCount>, BroadcastMapSendError<Vec<u8>>> =
        web_socket.try_send(broadcast_type, String::from("three").into_bytes());
    assert_eq!(from_str.unwrap(), Some(1));
    assert_eq!(from_string.unwrap(), Some(1));
    assert_eq!(from_bytes.unwrap(), Some(1));
    let first: Result<Vec<u8>, RecvError> = receiver.recv().await;
    let second: Result<Vec<u8>, RecvError> = receiver.recv().await;
    let third: Result<Vec<u8>, RecvError> = receiver.recv().await;
    assert_eq!(first.unwrap(), String::from("one").into_bytes());
    assert_eq!(second.unwrap(), String::from("two").into_bytes());
    assert_eq!(third.unwrap(), String::from("three").into_bytes());
}

#[test]
fn try_send_fails_and_hands_the_payload_back_when_no_subscriber_is_left() {
    let web_socket: WebSocket = WebSocket::new();
    let broadcast_type: BroadcastType<String> = BroadcastType::PointToGroup(String::from("room"));
    let key: String = BroadcastType::get_key(broadcast_type.clone());
    let receiver: BroadcastMapReceiver<Vec<u8>> = web_socket
        .get_broadcast_map()
        .subscribe_or_insert(&key, DEFAULT_BROADCAST_SENDER_CAPACITY);
    drop(receiver);
    let result: Result<Option<ReceiverCount>, BroadcastMapSendError<Vec<u8>>> =
        web_socket.try_send(broadcast_type, String::from("hello"));
    let error: BroadcastMapSendError<Vec<u8>> = result.unwrap_err();
    let message: String = error.to_string();
    assert_eq!(message, String::from("channel closed"));
    let recovered: Vec<u8> = error.0;
    assert_eq!(recovered, String::from("hello").into_bytes());
}

#[test]
fn send_returns_the_number_of_receivers() {
    let web_socket: WebSocket = WebSocket::new();
    let broadcast_type: BroadcastType<String> = BroadcastType::PointToGroup(String::from("room"));
    let key: String = BroadcastType::get_key(broadcast_type.clone());
    let _receiver: BroadcastMapReceiver<Vec<u8>> = web_socket
        .get_broadcast_map()
        .subscribe_or_insert(&key, DEFAULT_BROADCAST_SENDER_CAPACITY);
    let count: Option<ReceiverCount> = web_socket.send(broadcast_type, "hello");
    assert_eq!(count, Some(1));
}

#[test]
fn send_returns_none_for_a_channel_that_was_never_created() {
    let web_socket: WebSocket = WebSocket::new();
    let count: Option<ReceiverCount> = web_socket.send(
        BroadcastType::<String>::PointToGroup(String::from("room")),
        "hello",
    );
    assert_eq!(count, None);
}

#[test]
#[should_panic(expected = "SendError")]
fn send_panics_when_the_channel_has_no_subscriber() {
    let web_socket: WebSocket = WebSocket::new();
    let broadcast_type: BroadcastType<String> = BroadcastType::PointToGroup(String::from("room"));
    let key: String = BroadcastType::get_key(broadcast_type.clone());
    let receiver: BroadcastMapReceiver<Vec<u8>> = web_socket
        .get_broadcast_map()
        .subscribe_or_insert(&key, DEFAULT_BROADCAST_SENDER_CAPACITY);
    drop(receiver);
    let count: Option<ReceiverCount> = web_socket.send(broadcast_type, "hello");
    assert_eq!(count, None);
}

#[test]
fn unsubscribing_a_key_removes_the_whole_channel() {
    let web_socket: WebSocket = WebSocket::new();
    let broadcast_type: BroadcastType<String> = BroadcastType::PointToGroup(String::from("room"));
    let key: String = BroadcastType::get_key(broadcast_type.clone());
    let _receiver: BroadcastMapReceiver<Vec<u8>> = web_socket
        .get_broadcast_map()
        .subscribe_or_insert(&key, DEFAULT_BROADCAST_SENDER_CAPACITY);
    let removed: Option<Broadcast<Vec<u8>>> = web_socket.get_broadcast_map().unsubscribe(&key);
    assert!(removed.is_some());
    let count: ReceiverCount = web_socket.receiver_count(broadcast_type.clone());
    assert_eq!(count, 0);
    let result: Result<Option<ReceiverCount>, BroadcastMapSendError<Vec<u8>>> =
        web_socket.try_send(broadcast_type, "hello");
    let sent: Option<ReceiverCount> = result.unwrap();
    assert_eq!(sent, None);
    let removed_again: Option<Broadcast<Vec<u8>>> =
        web_socket.get_broadcast_map().unsubscribe(&key);
    assert!(removed_again.is_none());
}

#[test]
fn a_cloned_web_socket_keeps_counting_the_subscribers_of_the_original() {
    let web_socket: WebSocket = WebSocket::new();
    let broadcast_type: BroadcastType<String> = BroadcastType::PointToGroup(String::from("room"));
    let key: String = BroadcastType::get_key(broadcast_type.clone());
    let _receiver: BroadcastMapReceiver<Vec<u8>> = web_socket
        .get_broadcast_map()
        .subscribe_or_insert(&key, DEFAULT_BROADCAST_SENDER_CAPACITY);
    let clone: WebSocket = web_socket.clone();
    let count: ReceiverCount = clone.receiver_count(broadcast_type);
    assert_eq!(count, 1);
}

#[test]
fn a_cloned_web_socket_does_not_share_later_map_insertions() {
    let web_socket: WebSocket = WebSocket::new();
    let clone: WebSocket = web_socket.clone();
    let broadcast_type: BroadcastType<String> = BroadcastType::PointToGroup(String::from("room"));
    let key: String = BroadcastType::get_key(broadcast_type.clone());
    let _receiver: BroadcastMapReceiver<Vec<u8>> = clone
        .get_broadcast_map()
        .subscribe_or_insert(&key, DEFAULT_BROADCAST_SENDER_CAPACITY);
    let clone_count: ReceiverCount = clone.receiver_count(broadcast_type.clone());
    let original_count: ReceiverCount = web_socket.receiver_count(broadcast_type);
    assert_eq!(clone_count, 1);
    assert_eq!(original_count, 0);
}

#[test]
fn debug_renders_the_inner_broadcast_map_without_panicking() {
    let web_socket: WebSocket = WebSocket::new();
    let empty: String = format!("{:?}", web_socket);
    assert!(!empty.is_empty());
    let key: String =
        BroadcastType::<String>::get_key(BroadcastType::PointToGroup(String::from("room")));
    let _receiver: BroadcastMapReceiver<Vec<u8>> = web_socket
        .get_broadcast_map()
        .subscribe_or_insert(&key, DEFAULT_BROADCAST_SENDER_CAPACITY);
    let populated: String = format!("{:?}", web_socket);
    assert!(populated.contains("ptg--room"));
    assert_ne!(empty, populated);
}
