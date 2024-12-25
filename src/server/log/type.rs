use std::borrow::Cow;

use http_type::ArcRwLock;

pub type LogArcLock = ArcRwLock<Vec<Cow<'static, &'static str>>>;

pub struct Log {
    pub(crate) error: ArcRwLock<Vec<Cow<'static, &'static str>>>,
    pub(crate) info: ArcRwLock<Vec<Cow<'static, &'static str>>>,
    pub(crate) debug: ArcRwLock<Vec<Cow<'static, &'static str>>>,
}
