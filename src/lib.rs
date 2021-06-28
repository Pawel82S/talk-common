mod message;
mod user;

pub use message::Message;
pub use user::{User, UserID};

pub const COMM_PORT: u16 = 7878; // 7878 is Rust typed on phone keybord

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
