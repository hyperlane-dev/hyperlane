use super::error::Error;
use super::r#type::Request;
use http_constant::*;
use std::borrow::Cow;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;

impl<'a> Request<'a> {
    pub fn new(stream: &TcpStream) -> Result<Self, Error> {
        let mut reader: BufReader<&TcpStream> = BufReader::new(stream);
        let mut request_line: String = String::new();
        reader
            .read_line(&mut request_line)
            .map_err(|_| Error::HttpReadError)?;
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() < 3 {
            return Err(Error::InvalidHttpRequest);
        }
        let method: Cow<'_, str> = Cow::Owned(parts[0].to_string());
        let full_path: String = parts[1].to_string();
        let hash_index: Option<usize> = full_path.find(HASH_SYMBOL);
        let query_index: Option<usize> = full_path.find(QUERY_SYMBOL);
        let hash: Cow<'_, str> = hash_index.map_or(Cow::Borrowed(EMPTY_STR), |i| {
            let temp: String = full_path[i + 1..].to_string();
            temp.into()
        });
        let query: Cow<'_, str> = query_index.map_or(Cow::Borrowed(EMPTY_STR), |i| {
            let temp = full_path[i + 1..].to_string();
            if hash_index.is_none() || hash_index.unwrap() <= i {
                return temp.into();
            }
            let data = temp
                .split(HASH_SYMBOL)
                .next()
                .unwrap_or_default()
                .to_string();
            data.into()
        });
        let path: Cow<'_, str> = if let Some(i) = query_index.or(hash_index) {
            Cow::Owned(full_path[..i].to_string())
        } else {
            Cow::Owned(full_path)
        };
        let mut headers: HashMap<Cow<'a, str>, Cow<'a, str>> = HashMap::new();
        let mut host: Cow<'_, str> = Cow::Borrowed(EMPTY_STR);
        let mut content_length: usize = 0;
        loop {
            let mut header_line: String = String::new();
            reader
                .read_line(&mut header_line)
                .map_err(|_| Error::HttpReadError)?;
            let header_line: &str = header_line.trim();
            if header_line.is_empty() {
                break;
            }
            let parts: Vec<&str> = header_line.splitn(2, COLON_SPACE_SYMBOL).collect();
            if parts.len() != 2 {
                continue;
            }
            let key: Cow<'_, str> = Cow::Owned(parts[0].trim().to_string());
            let value: Cow<'_, str> = Cow::Owned(parts[1].trim().to_string());
            if key.eq_ignore_ascii_case(HOST) {
                host = value.clone();
            }
            if key.eq_ignore_ascii_case(CONTENT_LENGTH) {
                content_length = value.parse().unwrap_or(0);
            }
            headers.insert(key, value);
        }
        let mut body: Vec<u8> = Vec::new();
        if content_length > 0 {
            body.resize(content_length, 0);
            reader
                .read_exact(&mut body)
                .map_err(|_| Error::HttpReadError)?;
        }
        Ok(Request {
            method,
            host,
            path,
            query,
            hash,
            headers,
            body,
        })
    }
}
