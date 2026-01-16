//! Blob storage interfaces.
//!
//! Based on WASI blobstore.

use std::fmt;
use std::future::Future;

/// Blob storage errors.
#[derive(Debug)]
pub enum Error {
    ContainerNotFound(String),
    ObjectNotFound(String),
    ContainerExists(String),
    Store(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ContainerNotFound(name) => write!(f, "container not found: {}", name),
            Error::ObjectNotFound(name) => write!(f, "object not found: {}", name),
            Error::ContainerExists(name) => write!(f, "container already exists: {}", name),
            Error::Store(msg) => write!(f, "store error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

/// Metadata for a stored object.
#[derive(Debug, Clone)]
pub struct ObjectMeta {
    /// Object name/key.
    pub name: String,
    /// Size in bytes.
    pub size: u64,
    /// When the object was created (Unix timestamp).
    pub created_at: Option<u64>,
}

/// A blob storage container.
pub trait Container {
    /// Get object data.
    fn get(&self, name: &str) -> impl Future<Output = Result<Vec<u8>, Error>>;

    /// Store object data.
    fn put(&self, name: &str, data: &[u8]) -> impl Future<Output = Result<(), Error>>;

    /// Delete an object.
    fn delete(&self, name: &str) -> impl Future<Output = Result<(), Error>>;

    /// Check if an object exists.
    fn exists(&self, name: &str) -> impl Future<Output = Result<bool, Error>>;

    /// List objects in the container.
    fn list(&self) -> impl Future<Output = Result<Vec<ObjectMeta>, Error>>;

    /// Get object metadata.
    fn metadata(&self, name: &str) -> impl Future<Output = Result<ObjectMeta, Error>>;

    /// Copy an object within this container.
    fn copy(&self, src: &str, dst: &str) -> impl Future<Output = Result<(), Error>>;
}

/// A blob store that manages containers.
pub trait BlobStore {
    /// The container type.
    type Container: Container;

    /// Create a new container.
    fn create_container(&self, name: &str) -> impl Future<Output = Result<(), Error>>;

    /// Delete a container.
    fn delete_container(&self, name: &str) -> impl Future<Output = Result<(), Error>>;

    /// Get a container by name.
    fn container(&self, name: &str) -> impl Future<Output = Result<Self::Container, Error>>;

    /// Check if a container exists.
    fn container_exists(&self, name: &str) -> impl Future<Output = Result<bool, Error>>;

    /// List all containers.
    fn list_containers(&self) -> impl Future<Output = Result<Vec<String>, Error>>;
}
