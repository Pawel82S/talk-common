use std::time::SystemTime;

use crate::UserID;

pub struct Message {
    from: UserID,
    to: UserID,
    time: SystemTime,
    content: String,
}

impl Message {
    pub fn new(content: String, from: UserID, to: UserID) -> Self {
        Self {
            from,
            to,
            time: SystemTime::now(),
            content,
        }
    }

    pub fn from(&self) -> &UserID {
        &self.from
    }

    pub fn to(&self) -> &UserID {
        &self.to
    }

    pub fn time(&self) -> &SystemTime {
        &self.time
    }

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
