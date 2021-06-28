mod message;
mod user;

pub use message::Message;
pub use user::{User, UserID};

/// Port used for communication between client and server.
pub const COMM_PORT: u16 = 7878; // 7878 is Rust typed on phone keybord

/// This is how client and server are communicating.
pub enum Comm {
    /// This message server will send to every newly connected and not logged client in case user
    /// want to create new account this will be new ID reserved for time of creating new account.
    Connected(UserID),

    /// This message should be send by client every time it disconnects from server.
    Disconnnected(UserID),

    /// This must be send every time client want to authenticate with server.
    Login { id: UserID, password: String },

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
        new_password: String,
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

/// Comunnication errors.
pub enum CommError {
    /// Used only during login procedure.
    BadLoginData,

    /// Used when last communication with UserID only failed.
    InvalidUserId,

    /// Used only when changing password.
    InvalidPassword,

    /// Other invalid operation.
    InvalidOperation,
}
