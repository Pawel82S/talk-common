use std::time::SystemTime;

use crate::UserID;

/// Structure representing message.
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

    /// Returns reviever ID.
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
