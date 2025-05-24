use crate::*;

pub(crate) type ArcErrorHandle = Arc<dyn ErrorHandle + Send + Sync + 'static>;
