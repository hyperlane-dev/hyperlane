use std::{borrow::Cow, collections::HashMap};

#[derive(Debug, Clone)]
pub struct Response<'a> {
    pub(super) version: Cow<'a, str>,
    pub(super) status_code: u16,
    pub(super) reason_phrase: Cow<'a, str>,
    pub(super) headers: HashMap<Cow<'a, str>, Cow<'a, str>>,
    pub(super) body: Vec<u8>,
}
