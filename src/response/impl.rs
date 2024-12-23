use super::r#type::Response;
use http_constant::*;
use std::{borrow::Cow, collections::HashMap};

impl<'a> Default for Response<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Response<'a> {
    pub fn new() -> Self {
        Response {
            version: Cow::Borrowed(HTTP_VERSION_1_1),
            status_code: 200,
            reason_phrase: Cow::Borrowed(OK),
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }

    pub fn version<S: Into<Cow<'a, str>>>(&mut self, version: S) -> &mut Self {
        self.version = version.into();
        self
    }

    pub fn status_code(&mut self, code: u16) {
        self.status_code = code;
    }

    pub fn reason_phrase<S: Into<Cow<'a, str>>>(&mut self, phrase: S) -> &mut Self {
        self.reason_phrase = phrase.into();
        self
    }

    pub fn header<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        self.headers.insert(key.into(), value.into());
        self
    }

    pub fn body<B: Into<Vec<u8>>>(&mut self, body: B) -> &mut Self {
        self.body = body.into();
        self
    }

    pub fn build(&self) -> Vec<u8> {
        let mut response: String = String::new();
        response.push_str(&format!(
            "{} {} {}{}",
            self.version, self.status_code, self.reason_phrase, HTTP_BR
        ));
        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}{}", key, value, HTTP_BR));
        }
        response.push_str(HTTP_BR);
        let mut response_bytes: Vec<u8> = response.into_bytes();
        response_bytes.extend_from_slice(&self.body);
        response_bytes
    }
}
