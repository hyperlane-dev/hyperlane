use crate::*;

#[inline]
pub async fn send_response<T: Into<ResponseBody>>(
    arc_lock_controller_data: &ArcRwLockControllerData,
    status_code: usize,
    response_body: T,
) -> ResponseResult {
    let controller_data: RwLockWriteControllerData = arc_lock_controller_data.write().await;
    let mut response: Response = controller_data.get_response().clone();
    let body: ResponseBody = response_body.into();
    let stream_lock: ArcRwLockStream = controller_data.get_stream().clone().unwrap();
    let response_res: ResponseResult = response
        .set_body(body)
        .set_status_code(status_code)
        .send(&stream_lock)
        .await;
    response_res
}

#[inline]
pub async fn send_response_once<T: Into<ResponseBody>>(
    arc_lock_controller_data: &ArcRwLockControllerData,
    status_code: usize,
    response_body: T,
) -> ResponseResult {
    let controller_data: RwLockWriteControllerData = arc_lock_controller_data.write().await;
    let mut response: Response = controller_data.get_response().clone();
    let body: ResponseBody = response_body.into();
    let stream_lock: ArcRwLockStream = controller_data.get_stream().clone().unwrap();
    let response_res: ResponseResult = response
        .set_body(body)
        .set_status_code(status_code)
        .send(&stream_lock)
        .await;
    let _ = response.close(&stream_lock).await;
    response_res
}
