mod message;
mod user;

pub use message::Message;
pub use user::{User, UserID};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
