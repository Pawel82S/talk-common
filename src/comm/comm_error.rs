use crate::serialize::{Serialize, SerializeError};

/// Comunnication errors.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CommError {
    /// Used only during login procedure.
    BadLoginData,

    /// Used when last communication with UserID only failed.
    InvalidUserId,

    /// Used only when changing password.
    InvalidPassword,

    /// Other invalid operation.
    InvalidOperation,

    /// Unknown
    Unknown, // This should be last option
}

impl Serialize for CommError {
    type Item = CommError;

    /// Converts CommError to `u8` number and writes it to `buffer[0]`.
    fn serialize(&self, buffer: &mut [u8]) -> Result<(), SerializeError> {
        Ok(buffer[0] = *self as u8)
    }

    /// Reads `buffer[0]` and creates CommError object from it.
    fn deserialize(buffer: &[u8]) -> Result<Self::Item, SerializeError> {
        match buffer[0] {
            0 => Ok(CommError::BadLoginData),
            1 => Ok(CommError::InvalidUserId),
            2 => Ok(CommError::InvalidPassword),
            3 => Ok(CommError::InvalidOperation),
            sig => Err(SerializeError::UnknownSignature(sig)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bad_login_data() {
        let mut buffer = [0xFF];
        let e1 = CommError::BadLoginData;
        e1.serialize(&mut buffer).unwrap();
        let e2 = CommError::deserialize(&buffer).unwrap();
        assert_eq!(e1, e2);
    }

    #[test]
    fn invalid_user_id() {
        let mut buffer = [0xFF];
        let e1 = CommError::InvalidUserId;
        e1.serialize(&mut buffer).unwrap();
        let e2 = CommError::deserialize(&buffer).unwrap();
        assert_eq!(e1, e2);
    }

    #[test]
    fn invalid_password() {
        let mut buffer = [0xFF];
        let e1 = CommError::InvalidPassword;
        e1.serialize(&mut buffer).unwrap();
        let e2 = CommError::deserialize(&buffer).unwrap();
        assert_eq!(e1, e2);
    }

    #[test]
    fn invalid_operation() {
        let mut buffer = [0xFF];
        let e1 = CommError::InvalidOperation;
        e1.serialize(&mut buffer).unwrap();
        let e2 = CommError::deserialize(&buffer).unwrap();
        assert_eq!(e1, e2);
    }

    #[test]
    fn unknown_signature() {
        let buffer = [0xFF];
        match CommError::deserialize(&buffer) {
            Ok(_) => panic!(),
            Err(e) => assert_eq!(e, SerializeError::UnknownSignature(0xFF)),
        }
    }
}
