mod comm_error;

use crate::{
    serialize::{Serialize, SerializeError},
    Message, User, UserID, MAX_PASS_BYTE_LEN,
};
pub use comm_error::CommError;

/// Communication inteterface between `talk-client` and `talk-server`.
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
    /// and stored there until reciver will log in. Then server will send messagess one by one
    /// (starting from oldest), waiting every time for client to send `Comm::Accept` message. If
    /// server or client wont `Comm::Accept` message then message wasn't recieved and will remain
    /// at server (and should remain at client) to send another time.
    Message(Message),

    /// This is used when user is logged. Client should never send `User` struct to server or it
    /// will be recjeted.
    AddInvitation(UserID),

    /// This is used when user is logged. Client should never send `User` struct to server or it
    /// will be recjeted.
    RemoveInvitation(UserID),

    /// This is used when user is logged. Client should never send `User` struct to server or it
    /// will be recjeted.
    AddFriend(UserID),

    /// This is used when user is logged. Client should never send `User` struct to server or it
    /// will be recjeted.
    RemoveFriend(UserID),
}

impl Serialize for Comm {
    type Item = Comm;

    /// Writes Comm to `buffer`. Returns `()` on success or `SerializeError` otherwise.
    fn serialize(&self, buffer: &mut [u8]) -> Result<(), SerializeError> {
        // TODO: Add buffer size checks to each condition so we won't panic here but return proper
        // SerializeError. For now it will be just MVP that panic when we try to write too much.
        match self {
            Comm::Connected(id) => {
                buffer[0] = 0;
                crate::write_bytes_to_buffer(&mut buffer[1..], &id.to_ne_bytes());
            }

            Comm::Disconnected(id) => {
                buffer[0] = 1;
                crate::write_bytes_to_buffer(&mut buffer[1..], &id.to_ne_bytes());
            }

            Comm::Login { id, password } => {
                buffer[0] = 2;
                let mut index = 1;
                index += crate::write_bytes_to_buffer(&mut buffer[index..], &id.to_ne_bytes());
                crate::write_bytes_to_buffer(&mut buffer[index..], password.as_bytes());
            }

            Comm::Accepted => buffer[0] = 3,

            Comm::Rejected(err) => {
                buffer[0] = 4;
                err.serialize(&mut buffer[1..])?
            }

            Comm::User(user) => {
                buffer[0] = 5;
                user.serialize(&mut buffer[1..])?
            }

            Comm::ChangePassword {
                new_password,
                old_password,
            } => {
                buffer[0] = 6;
                let np = new_password.as_bytes();
                let op = old_password.as_bytes();
                let index = 1 + crate::MAX_PASS_BYTE_LEN;

                crate::write_bytes_to_buffer(&mut buffer[1..index], np);
                // Each password must have reserved exactly the same number of bytes.
                crate::write_bytes_to_buffer(
                    &mut buffer[index..index + crate::MAX_PASS_BYTE_LEN],
                    op,
                );
            }

            Comm::Message(msg) => {
                buffer[0] = 7;
                msg.serialize(&mut buffer[1..])?
            }

            Comm::AddInvitation(id) => {
                buffer[0] = 8;
                crate::write_bytes_to_buffer(&mut buffer[1..], &id.to_ne_bytes());
            }

            Comm::RemoveInvitation(id) => {
                buffer[0] = 9;
                crate::write_bytes_to_buffer(&mut buffer[1..], &id.to_ne_bytes());
            }

            Comm::AddFriend(id) => {
                buffer[0] = 10;
                crate::write_bytes_to_buffer(&mut buffer[1..], &id.to_ne_bytes());
            }

            Comm::RemoveFriend(id) => {
                buffer[0] = 11;
                crate::write_bytes_to_buffer(&mut buffer[1..], &id.to_ne_bytes());
            }
        }

        Ok(())
    }

    /// Reads Comm from `buffer`. Returns `Self` on success or `SerializeError` otherwise.
    fn deserialize(buffer: &[u8]) -> Result<Self::Item, SerializeError> {
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
                let password =
                    crate::parse_string_from_bytes(&buffer[9..9 + MAX_PASS_BYTE_LEN]).to_string();
                Ok(Comm::Login { id, password })
            }

            // Comm::Accepted
            3 => Ok(Comm::Accepted),

            // Comm::Rejected
            4 => Ok(Comm::Rejected(
                CommError::deserialize(&buffer[1..]).unwrap(),
            )),

            // Comm::User
            5 => Ok(Comm::User(User::deserialize(&buffer[1..]).unwrap())),

            // Comm::ChangePassword
            6 => {
                let index = 1 + MAX_PASS_BYTE_LEN;
                let new_password = crate::parse_string_from_bytes(&buffer[1..index]).to_string();
                let old_password =
                    crate::parse_string_from_bytes(&buffer[index..index + MAX_PASS_BYTE_LEN])
                        .to_string();
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
            sig => Err(SerializeError::UnknownSignature(sig)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comm_connect() {
        let buffer = [0u8, 1, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(Comm::deserialize(&buffer).unwrap(), Comm::Connected(1));
        assert_ne!(Comm::deserialize(&buffer).unwrap(), Comm::Connected(2));
    }

    #[test]
    fn comm_disconnected() {
        let buffer = [1u8, 1, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(Comm::deserialize(&buffer).unwrap(), Comm::Disconnected(1));
        assert_ne!(Comm::deserialize(&buffer).unwrap(), Comm::Disconnected(2));
    }
}
