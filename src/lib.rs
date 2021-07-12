//! Defines data types and constants that are used by talk-client and talk-server. It
//! doesn't do anything on it's own.

#[warn(missing_docs)]
mod comm;
mod message;
mod serialize;
mod user;

pub use comm::{Comm, CommError};
pub use message::Message;
pub use serialize::{Serialize, SerializeError};
use std::{convert::TryInto, mem, str};
pub use user::User;

// NOTE: I've created separate type in case we want to change it for something more advanced in the
// future.
pub type UserID = u64;

/// Size in bytes of UserID
pub const USER_ID_SIZE: usize = mem::size_of::<UserID>();

/// Port used for communication between client and server.
pub const COMM_PORT: u16 = 7878; // 7878 is Rust typed on phone keybord

/// Size of network buffer in bytes.
pub const NET_BUFF_SIZE: usize = 512;

/// Minimum user password length in characters, not bytes. Unicode characters can take more than
/// one byte of memory.
pub const MIN_PASS_CHAR_LEN: usize = 4;

/// Maximum user password length in bytes, not characters. Unicode characters can take more than
/// one byte of memory. We are using bytes here instead of characters because we must know how many
/// bytes it will take to save/load to/from file or transfer thru network.
pub const MAX_PASS_BYTE_LEN: usize = 30;

/// Returns UserID from a slice of bytes.
// TODO: This function should propably return Result in case of parsing error.
pub fn parse_id_from_bytes(bytes: &[u8]) -> UserID {
    u64::from_ne_bytes(bytes[..USER_ID_SIZE].try_into().unwrap())
}

/// Returns String from a slice of bytes or empty String if there was an error.
// TODO: This function should propably return Result in case of parsing error.
pub fn parse_string_from_bytes(bytes: &[u8]) -> &str {
    str::from_utf8(bytes.split(|&c| c == 0).next().unwrap_or_default()).unwrap_or_default()
}

/// Writes bytes to buffer one by one, and returns number of bytes written.
pub fn write_bytes_to_buffer(buffer: &mut [u8], bytes: &[u8]) -> usize {
    let mut index = 0;
    for byte in bytes {
        buffer[index] = *byte;
        index += 1;
    }

    index
}
