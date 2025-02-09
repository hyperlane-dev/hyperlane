use crate::*;

#[inline]
pub fn send_response<T: Into<Vec<u8>>>(
    arc_lock_controller_data: &ArcRwLockControllerData,
    status_code: usize,
    response_data: T,
) -> ResponseResult {
    let controller_data: RwLockWriteControllerData = arc_lock_controller_data.write().unwrap();
    let mut response: Response = controller_data.get_response().clone();
    let body: Vec<u8> = response_data.into();
    let stream: ArcTcpStream = controller_data.get_stream().clone().unwrap();
    let response_res: ResponseResult = response
        .set_body(body)
        .set_status_code(status_code)
        .send(&stream);
    response_res
}

#[inline]
pub fn send_response_once<T: Into<Vec<u8>>>(
    arc_lock_controller_data: &ArcRwLockControllerData,
    status_code: usize,
    response_data: T,
) -> ResponseResult {
    let controller_data: RwLockWriteControllerData = arc_lock_controller_data.write().unwrap();
    let mut response: Response = controller_data.get_response().clone();
    let body: Vec<u8> = response_data.into();
    let stream: ArcTcpStream = controller_data.get_stream().clone().unwrap();
    let response_res: ResponseResult = response
        .set_body(body)
        .set_status_code(status_code)
        .send(&stream);
    let _ = response.close(&stream);
    response_res
}
