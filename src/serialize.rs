/// This trait should be implemented by every object that requires to be serialized.
pub trait Serialize {
    type Item;

    /// Serializing object to u8 buffer. Returns empty tuple (nothing) on success or
    /// `SerializeError`.
    fn serialize(&self, buffer: &mut [u8]) -> Result<(), SerializeError>;

    /// Deserializing object from u8 buffer. Returns `Self` on success or `SerializeError`.
    fn deserialize(buffer: &[u8]) -> Result<Self::Item, SerializeError>;
}

/// This type describes errors that can occur when data is serialized/deserialized. They are not
/// transmitted between client and server.
#[derive(Debug, PartialEq)]
pub enum SerializeError {
    /// It occurs at reading when some unexpected data has been read.
    UnknownSignature(u8),

    /// Buffer is too small to fit data that we need to read or write.
    NotEnoughData,
}
