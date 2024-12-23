#[derive(Debug)]
pub enum Error {
    TcpBindError,
    HttpReadError,
    InvalidHttpRequest,
    Unknown,
}
