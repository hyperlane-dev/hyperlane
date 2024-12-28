use std::fmt::Debug;

pub trait LogFuncTrait: Fn(&String) -> String + Send + Sync + 'static {}
impl<T> LogFuncTrait for T where T: Fn(&String) -> String + Send + Sync + 'static {}
pub trait LogDataTrait: Debug {}
impl<T> LogDataTrait for T where T: Debug + 'static {}
