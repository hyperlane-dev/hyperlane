use super::*;

/// A thread-safe reference-counted `TcpStream`.
pub type ArcStream = Arc<TcpStream>;

/// A socket host represented by an IP address.
pub type SocketHost = IpAddr;

/// A socket port number.
pub type SocketPort = u16;
