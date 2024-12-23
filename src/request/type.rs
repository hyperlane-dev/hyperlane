use std::{borrow::Cow, collections::HashMap};

#[derive(Debug, Clone)]
pub struct Request<'a> {
    pub(crate) method: Cow<'a, str>,
    pub(crate) host: Cow<'a, str>,
    pub(crate) path: Cow<'a, str>,
    pub(crate) query: Cow<'a, str>,
    pub(crate) hash: Cow<'a, str>,
    pub(crate) headers: HashMap<Cow<'a, str>, Cow<'a, str>>,
    pub(crate) body: Vec<u8>,
}
