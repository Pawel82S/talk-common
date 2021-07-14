use std::time::SystemTime;

use crate::{
    serialize::{Serialize, SerializeError},
    UserID,
};

/// Represents message that can be sent between users.
#[derive(Debug, PartialEq)]
pub struct Message {
    from: UserID,
    to: UserID,
    // NOTE: In future we shuld have also Time Zone here.
    time: SystemTime,
    content: String,
}

impl Message {
    /// Creates new message with current system time.
    pub fn new(content: String, from: UserID, to: UserID) -> Self {
        Self {
            from,
            to,
            time: SystemTime::now(),
            content,
        }
    }

    /// Returns sender ID.
    pub fn from(&self) -> &UserID {
        &self.from
    }

    /// Returns reciever ID.
    pub fn to(&self) -> &UserID {
        &self.to
    }

    /// Time when message was sent.
    pub fn time(&self) -> &SystemTime {
        &self.time
    }

    /// Message contents.
    pub fn content(&self) -> &String {
        &self.content
    }
}

impl Serialize for Message {
    type Item = Message;

    fn serialize(&self, buffer: &mut [u8]) -> Result<(), SerializeError> {
        // TODO: Add buffer size checks and return appropirate SerializeError instead of panic from
        // accessing index out of buffer slice. This is done this way for now as a MVP.
        let mut index = crate::write_bytes_to_buffer(
            &mut buffer[..crate::USER_ID_SIZE],
            &self.from.to_ne_bytes(),
        );
        index += crate::write_bytes_to_buffer(&mut buffer[index..], &self.to.to_ne_bytes());

        // TODO: Serialize SystemTime here.

        crate::write_bytes_to_buffer(&mut buffer[index..], self.content.as_bytes());

        Ok(())
    }

    fn deserialize(buffer: &[u8]) -> Result<Self::Item, SerializeError> {
        // TODO: Add buffer size checks and return appropirate SerializeError instead of panic from
        // accessing index out of buffer slice. This is done this way for now as a MVP.
        let from = crate::parse_id_from_bytes(&buffer[..crate::USER_ID_SIZE]);
        let index = 2 * crate::USER_ID_SIZE;
        let to = crate::parse_id_from_bytes(&buffer[crate::USER_ID_SIZE..index]);

        // TODO: Deserialize SystemTime here.

        let content = crate::parse_string_from_bytes(&buffer[index..]).to_string();

        Ok(Self {
            from,
            to,
            time: SystemTime::now(),
            content,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content() {
        let content = "Test message".to_string();
        let from = 1;
        let to = 0;
        let message = Message::new(content.clone(), from, to);

        assert_eq!(message.content(), &content);
        assert_eq!(message.from(), &from);
        assert_eq!(message.to(), &to);
        assert_ne!(message.time(), &SystemTime::now());
    }
}
