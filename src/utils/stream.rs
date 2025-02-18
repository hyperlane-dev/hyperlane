use crate::*;

pub async fn get_stream(
    arc_lock_controller_data: &ArcRwLockControllerData,
) -> OptionArcRwLockStream {
    let controller_data: ControllerData = get_controller_data(&arc_lock_controller_data).await;
    controller_data.get_stream().clone()
}

pub async fn get_socket_addr(arc_lock_controller_data: &ArcRwLockControllerData) -> Option<String> {
    let stream_result: OptionArcRwLockStream = get_stream(arc_lock_controller_data).await;
    if stream_result.is_none() {
        return None;
    }
    let socket_addr: String = stream_result
        .unwrap()
        .read()
        .await
        .peer_addr()
        .and_then(|host| Ok(host.to_string()))
        .unwrap_or_default();
    Some(socket_addr)
}
