/// This type describes errors that can occur when data is read or written to buffer. They are not
/// transmitted between client and server.
#[derive(Debug)]
pub enum CommParseError {
    /// It occurs at reading when some unexpected data has been read.
    UnknownSignature(u8),

    /// Buffer is too small to fit data that we need to read or write.
    NotEnoughData,
}
