//! CLI interfaces.
//!
//! Based on WASI CLI.

/// Access to command-line arguments.
pub trait Args {
    /// Returns an iterator over the command-line arguments.
    fn args(&self) -> impl Iterator<Item = String>;
}

/// Access to environment variables.
pub trait Environment {
    /// Returns an iterator over environment variables.
    fn vars(&self) -> impl Iterator<Item = (String, String)>;

    /// Gets the value of an environment variable.
    fn var(&self, key: &str) -> Option<String>;
}

/// Standard input stream.
pub trait Stdin {
    /// Read bytes from stdin.
    fn read(&self, buf: &mut [u8]) -> std::io::Result<usize>;
}

/// Standard output stream.
pub trait Stdout {
    /// Write bytes to stdout.
    fn write(&self, buf: &[u8]) -> std::io::Result<usize>;

    /// Flush stdout.
    fn flush(&self) -> std::io::Result<()>;
}

/// Standard error stream.
pub trait Stderr {
    /// Write bytes to stderr.
    fn write(&self, buf: &[u8]) -> std::io::Result<usize>;

    /// Flush stderr.
    fn flush(&self) -> std::io::Result<()>;
}
