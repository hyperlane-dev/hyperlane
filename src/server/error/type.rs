#[derive(Debug)]
pub enum Error {
    TcpBindError(String),
    HttpReadError(String),
    InvalidHttpRequest(http_type::RequestError),
    Unknown,
}
