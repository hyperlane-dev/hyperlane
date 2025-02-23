use crate::*;

pub type BoxFunc = Box<dyn Func + Sync + Send + 'static>;
pub type VecBoxFunc = Vec<BoxFunc>;
