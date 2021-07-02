mod comm_error;
mod comm_parse_error;

use crate::{Message, User, UserID, MAX_PASS_BYTE_LEN};
pub use comm_error::CommError;
pub use comm_parse_error::CommParseError;

/// This is how client and server are communicating.
#[derive(Debug, PartialEq)]
pub enum Comm {
    /// This message server will send to every newly connected and not logged client in case user
    /// want to create new account this will be new ID reserved for time of creating new account.
    Connected(UserID),

    /// This message should be send by client every time it disconnects from server.
    Disconnected(UserID),

    /// This must be send every time client want to authenticate with server.
    Login {
        /// This is `UserID`.
        id: UserID,
        /// This is `User` password.
        password: String,
    },

    /// Used every time when client or server should confirm operation without returning any data
    /// back.
    Accepted,

    /// When server cannot comply with client request it will return Rejected enum.
    Rejected(CommError),

    /// This will be send to client after successfull authentication. Client should never send back
    /// this to server. This will cause Rejected answer.
    User(User),

    /// Client can use it to change password on server. Server will return Accepted on success or
    /// CommError::InvalidPassword otherwise.
    ChangePassword {
        /// New `User` password.
        new_password: String,
        /// Current `User` password.
        old_password: String,
    },

    /// Every time clients want to send message they must use this. All messages are send to server
    /// and stored there until reciver will log in. Then server will send messagess one by one,
    /// waiting every time for client Accepted message. If server or client wont Accept message
    /// then message wasn't recieved.
    Message(Message),

    /// This is using when user is logged. Client should never send User struct to server.
    AddInvitation(UserID),

    /// This is using when user is logged. Client should never send User struct to server.
    RemoveInvitation(UserID),

    /// This is using when user is logged. Client should never send User struct to server.
    AddFriend(UserID),

    /// This is using when user is logged. Client should never send User struct to server.
    RemoveFriend(UserID),
}

impl Comm {
    /// Writes Comm to `buffer`. Returns `()` on success or `Err(CommParseError)` otherwise.
    pub fn try_into(&self, buffer: &mut [u8]) -> Result<(), CommParseError> {
        //match *self {
        //Comm::Connected(id) => (),
        //}
        Ok(())
    }

    /// Reads Comm from `buffer`. Returns `Self` on success or `Err(CommParseError)` otherwise.
    pub fn try_from(buffer: &[u8]) -> Result<Self, CommParseError> {
        match buffer[0] {
            // Comm::Connected
            0 => {
                let id = crate::parse_id_from_bytes(&buffer[1..9]);
                Ok(Comm::Connected(id))
            }

            // Comm::Disconnnected
            1 => {
                let id = crate::parse_id_from_bytes(&buffer[1..9]);
                Ok(Comm::Disconnected(id))
            }

            // Comm::Login
            2 => {
                let id = crate::parse_id_from_bytes(&buffer[1..9]);
                let password = crate::parse_string_from_bytes(&buffer[9..9 + MAX_PASS_BYTE_LEN]);
                Ok(Comm::Login { id, password })
            }

            // Comm::Accepted
            3 => Ok(Comm::Accepted),

            // Comm::Rejected
            4 => Ok(Comm::Rejected(CommError::try_from(&buffer[1..]))),

            // Comm::User
            5 => unimplemented!(),

            // Comm::ChangePassword
            6 => {
                let index = 1 + MAX_PASS_BYTE_LEN;
                let new_password = crate::parse_string_from_bytes(&buffer[1..index]);
                let old_password =
                    crate::parse_string_from_bytes(&buffer[index..index + MAX_PASS_BYTE_LEN]);
                Ok(Comm::ChangePassword {
                    new_password,
                    old_password,
                })
            }

            // Comm::Message
            7 => unimplemented!(),

            // Comm::AddInvitation
            8 => {
                let id = crate::parse_id_from_bytes(&buffer[1..]);
                Ok(Comm::AddInvitation(id))
            }

            // Comm::RemoveInvitation
            9 => {
                let id = crate::parse_id_from_bytes(&buffer[1..]);
                Ok(Comm::RemoveInvitation(id))
            }

            // Comm::AddFriend
            10 => {
                let id = crate::parse_id_from_bytes(&buffer[1..]);
                Ok(Comm::AddFriend(id))
            }

            // Comm::RemoveFriend
            11 => {
                let id = crate::parse_id_from_bytes(&buffer[1..]);
                Ok(Comm::RemoveFriend(id))
            }

            // Unknown Comm signature
            sig => Err(CommParseError::UnknownSignature(sig)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comm_connect() {
        let buffer = [0u8, 1, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(Comm::try_from(&buffer).unwrap(), Comm::Connected(1));
        assert_ne!(Comm::try_from(&buffer).unwrap(), Comm::Connected(2));
    }

    #[test]
    fn comm_disconnected() {
        let buffer = [1u8, 1, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(Comm::try_from(&buffer).unwrap(), Comm::Disconnected(1));
        assert_ne!(Comm::try_from(&buffer).unwrap(), Comm::Disconnected(2));
    }
}
