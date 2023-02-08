
use bytes::Bytes;

/// A frame in the Redis protocol.
#[derive(Clone, Debug)]
pub enum Body {
    String(String),
    Error(String),
    Number(u64),
    Bulk(Bytes),
    Null,
}
