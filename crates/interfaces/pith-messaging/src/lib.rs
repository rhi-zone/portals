//! Message queue interfaces.
//!
//! Based on WASI messaging.

use std::future::Future;
use std::time::Duration;

/// Messaging errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("channel closed")]
    Closed,
    #[error("timeout")]
    Timeout,
    #[error("messaging error: {0}")]
    Other(String),
}

/// A message with payload and metadata.
#[derive(Debug, Clone)]
pub struct Message {
    /// Message payload.
    pub data: Vec<u8>,
    /// Optional metadata/headers.
    pub metadata: Vec<(String, String)>,
}

impl Message {
    /// Create a new message with data.
    pub fn new(data: impl Into<Vec<u8>>) -> Self {
        Self {
            data: data.into(),
            metadata: Vec::new(),
        }
    }

    /// Add metadata to the message.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.push((key.into(), value.into()));
        self
    }
}

/// A message sender.
pub trait Sender {
    /// Send a message.
    fn send(&self, message: Message) -> impl Future<Output = Result<(), Error>>;
}

/// A message receiver.
pub trait Receiver {
    /// Receive a message, waiting indefinitely.
    fn receive(&self) -> impl Future<Output = Result<Message, Error>>;

    /// Receive a message with timeout.
    fn receive_timeout(&self, timeout: Duration) -> impl Future<Output = Result<Message, Error>>;

    /// Try to receive a message without blocking.
    fn try_receive(&self) -> impl Future<Output = Result<Option<Message>, Error>>;
}

/// A channel for point-to-point messaging.
pub trait Channel {
    /// The sender type.
    type Sender: Sender;
    /// The receiver type.
    type Receiver: Receiver;

    /// Create a new channel.
    fn create(&self) -> (Self::Sender, Self::Receiver);
}

/// A subscriber that receives messages from a topic.
pub trait Subscriber: Receiver {
    /// Unsubscribe from the topic.
    fn unsubscribe(self) -> impl Future<Output = Result<(), Error>>;
}

/// A topic for publish/subscribe messaging.
pub trait Topic {
    /// The subscriber type.
    type Subscriber: Subscriber;

    /// Publish a message to all subscribers.
    fn publish(&self, message: Message) -> impl Future<Output = Result<(), Error>>;

    /// Subscribe to receive messages.
    fn subscribe(&self) -> impl Future<Output = Result<Self::Subscriber, Error>>;
}

/// A messaging system that provides channels and topics.
pub trait Messaging {
    /// The channel type.
    type Channel: Channel;
    /// The topic type.
    type Topic: Topic;

    /// Create a new channel.
    fn channel(&self) -> Self::Channel;

    /// Get or create a topic by name.
    fn topic(&self, name: &str) -> impl Future<Output = Result<Self::Topic, Error>>;
}
