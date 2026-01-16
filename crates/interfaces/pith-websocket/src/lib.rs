//! WebSocket interfaces.
//!
//! Based on RFC 6455.

use std::fmt;
use std::future::Future;

/// WebSocket errors.
#[derive(Debug)]
pub enum Error {
    ConnectionFailed(String),
    SendFailed,
    Closed,
    Protocol(String),
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ConnectionFailed(msg) => write!(f, "connection failed: {}", msg),
            Error::SendFailed => write!(f, "send failed"),
            Error::Closed => write!(f, "connection closed"),
            Error::Protocol(msg) => write!(f, "protocol error: {}", msg),
            Error::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for Error {}

/// A WebSocket message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    /// Text message.
    Text(String),
    /// Binary message.
    Binary(Vec<u8>),
    /// Ping message.
    Ping(Vec<u8>),
    /// Pong message.
    Pong(Vec<u8>),
    /// Close message.
    Close,
}

/// A WebSocket client connection.
pub trait WebSocketClient {
    /// Send a message.
    fn send(&mut self, msg: Message) -> impl Future<Output = Result<(), Error>>;

    /// Receive the next message.
    fn recv(&mut self) -> impl Future<Output = Result<Message, Error>>;

    /// Close the connection.
    fn close(&mut self) -> impl Future<Output = Result<(), Error>>;
}

/// A WebSocket client connector.
pub trait WebSocketConnector {
    type Client: WebSocketClient;

    /// Connect to a WebSocket server.
    fn connect(&self, url: &str) -> impl Future<Output = Result<Self::Client, Error>>;
}
