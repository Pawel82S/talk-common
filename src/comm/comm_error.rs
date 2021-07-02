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

impl CommError {
    /// Converts CommError to `u8` number and writes it to `buffer[0]`.
    pub fn try_into(&self, buffer: &mut [u8]) {
        buffer[0] = *self as u8;
    }

    /// Reads `buffer[0]` and creates CommError object from it.
    pub fn try_from(buffer: &[u8]) -> Self {
        match buffer[0] {
            0 => CommError::BadLoginData,
            1 => CommError::InvalidUserId,
            2 => CommError::InvalidPassword,
            3 => CommError::InvalidOperation,
            _ => CommError::Unknown,
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
        e1.try_into(&mut buffer);
        let e2 = CommError::try_from(&buffer);
        assert_eq!(e1, e2);
    }

    #[test]
    fn invalid_user_id() {
        let mut buffer = [0xFF];
        let e1 = CommError::InvalidUserId;
        e1.try_into(&mut buffer);
        let e2 = CommError::try_from(&buffer);
        assert_eq!(e1, e2);
    }

    #[test]
    fn invalid_password() {
        let mut buffer = [0xFF];
        let e1 = CommError::InvalidPassword;
        e1.try_into(&mut buffer);
        let e2 = CommError::try_from(&buffer);
        assert_eq!(e1, e2);
    }

    #[test]
    fn invalid_operation() {
        let mut buffer = [0xFF];
        let e1 = CommError::InvalidOperation;
        e1.try_into(&mut buffer);
        let e2 = CommError::try_from(&buffer);
        assert_eq!(e1, e2);
    }

    #[test]
    fn unknown() {
        let buffer = [0xFF];
        let e1 = CommError::try_from(&buffer);
        assert_eq!(e1, CommError::Unknown);
    }
}
