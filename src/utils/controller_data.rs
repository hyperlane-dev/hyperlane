use crate::*;

#[inline]
pub async fn get_rw_lock_read_controller_data(
    arc_lock_controller_data: &ArcRwLockControllerData,
) -> RwLockReadControllerData {
    let controller_data: RwLockReadControllerData = arc_lock_controller_data.read().await;
    controller_data
}

#[inline]
pub async fn get_rw_lock_write_controller_data(
    arc_lock_controller_data: &ArcRwLockControllerData,
) -> RwLockWriteControllerData {
    let controller_data: RwLockWriteControllerData = arc_lock_controller_data.write().await;
    controller_data
}

#[inline]
pub async fn get_controller_data(
    arc_lock_controller_data: &ArcRwLockControllerData,
) -> ControllerData {
    let controller_data: ControllerData =
        get_rw_lock_read_controller_data(arc_lock_controller_data)
            .await
            .clone();
    controller_data
}
