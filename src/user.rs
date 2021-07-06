use crate::{CommParseError, UserID, USER_ID_SIZE};
use std::collections::HashSet;

/// Represents user.
#[derive(Debug, PartialEq)]
pub struct User {
    id: UserID,
    password: String,
    friends: HashSet<UserID>,
    invitations: HashSet<UserID>,
    // NOTE: In future we should have Time Zone information included.
}

impl User {
    // id = 8, password = MAX_PASS_BYTE_LEN, friends and invitations booth 1 byte each.
    const MIN_BYTE_LEN: usize = USER_ID_SIZE + crate::MAX_PASS_BYTE_LEN + 2;

    /// Creates empty user.
    pub fn new(id: UserID, password: String) -> Self {
        Self {
            id,
            password,
            friends: HashSet::new(),
            invitations: HashSet::new(),
        }
    }

    /// Tries to parse user data to u8 slice. It returns `()` on success and `CommParseError` on
    /// any error.
    pub fn try_into(&self, buffer: &mut [u8]) -> Result<(), CommParseError> {
        if buffer.len() < User::MIN_BYTE_LEN {
            Err(CommParseError::NotEnoughData)
        } else {
            let mut buffer_index =
                crate::write_bytes_to_buffer(&mut buffer[..USER_ID_SIZE], &self.id.to_ne_bytes());

            crate::write_bytes_to_buffer(&mut buffer[buffer_index..], self.password.as_bytes());
            // It doesn't matter if password is shorter than maximum length. Rest space is
            // reserved.
            buffer_index += crate::MAX_PASS_BYTE_LEN;

            // Now we have to write how many friends and invitations user have. Both are u8 (0-255)
            // which should be more than enough for this simple communicator.
            // TODO: Fix this algorithm to only count indexes that can fit in buffer. For now with
            // just few contacts this isn't problem, but it can be.
            buffer[buffer_index] = self.friends.len() as u8;
            buffer_index += 1;
            buffer[buffer_index] = self.invitations.len() as u8;
            buffer_index += 1;

            'id_write: for id_set in [self.friends.iter(), self.invitations.iter()] {
                for id in id_set {
                    if buffer.len() - buffer_index < USER_ID_SIZE {
                        break 'id_write;
                    }

                    buffer_index += crate::write_bytes_to_buffer(
                        &mut buffer[buffer_index..],
                        &id.to_ne_bytes(),
                    );
                }
            }

            Ok(())
        }
    }

    /// Tries to parse user data from u8 slice. It returns `Self` on success and `CommParseError` on
    /// any error.
    pub fn try_from(buffer: &[u8]) -> Result<Self, CommParseError> {
        if buffer.len() < User::MIN_BYTE_LEN {
            Err(CommParseError::NotEnoughData)
        } else {
            let id = crate::parse_id_from_bytes(&buffer[..USER_ID_SIZE]);
            let mut buffer_index = USER_ID_SIZE + crate::MAX_PASS_BYTE_LEN;
            let password =
                crate::parse_string_from_bytes(&buffer[USER_ID_SIZE..buffer_index]).to_string();
            let friends_count = buffer[buffer_index] as usize;
            buffer_index += 1;
            let invitations_count = buffer[buffer_index] as usize;
            buffer_index += 1;
            let mut friends = HashSet::new();
            let mut invitations = HashSet::new();

            // TODO: Add checks for buffer boundry so we cannot read outside buffer and cause
            // panic.
            let mut is_parsing_friends = true;
            for count in [friends_count, invitations_count] {
                for _ in 0..count {
                    let contact_id = crate::parse_id_from_bytes(
                        &buffer[buffer_index..buffer_index + USER_ID_SIZE],
                    );
                    if is_parsing_friends {
                        friends.insert(contact_id);
                    } else {
                        invitations.insert(contact_id);
                    }
                    buffer_index += USER_ID_SIZE;
                }

                is_parsing_friends = false;
            }

            Ok(User {
                id,
                password,
                friends,
                invitations,
            })
        }
    }

    /// Returns UserID number.
    pub fn id(&self) -> UserID {
        self.id
    }

    /// Returns user password.
    pub fn password(&self) -> &String {
        &self.password
    }

    /// Changes user password from current to new if current is the same as 'self.password'.
    pub fn change_password(&mut self, new_password: String, current_password: &str) -> bool {
        if self.password == current_password {
            self.password = new_password;
            true
        } else {
            false
        }
    }

    /// Returns set of user friends IDs
    pub fn friends(&self) -> &HashSet<UserID> {
        &self.friends
    }

    /// Returns true if user is friend with id.
    pub fn has_friend(&self, id: &UserID) -> bool {
        self.friends.contains(id)
    }

    /// Adds UserID to friends set. Returns true if id didn't existed and false otherwise.
    pub fn add_friend(&mut self, id: UserID) -> bool {
        self.friends.insert(id)
    }

    /// Removes 'id' from friends set. Returns true if it was removed, false otherwise.
    pub fn remove_friend(&mut self, id: UserID) -> bool {
        self.friends.remove(&id)
    }

    /// Returns set of UserID that have send invitations.
    pub fn invitations(&self) -> &HashSet<UserID> {
        &self.invitations
    }

    /// Returns true if user has invitation from 'id', false otherwise.
    pub fn has_invitation(&self, id: &UserID) -> bool {
        self.invitations.contains(id)
    }

    /// Adds UserID invitation set. Returns true if id didn't existed and false otherwise.
    pub fn add_invitation(&mut self, id: UserID) -> bool {
        self.invitations.insert(id)
    }

    /// Removes 'id' from invitation set. Returns true if it was removed, false otherwise.
    pub fn remove_invitation(&mut self, id: UserID) -> bool {
        self.invitations.remove(&id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn change_password() {
        let original_password = "abcd".to_string();
        let new_password = "new_password".to_string();
        let mut user = User::new(0, original_password.clone());

        // We shouldn't be able to change password if we don't provide old one correct
        assert!(!user.change_password(new_password.clone(), &"bad_password".to_string()));
        assert_eq!(user.password(), &original_password);

        // When we provide proper old password then we can change it to new one
        assert!(user.change_password(new_password, &original_password));
        assert_ne!(user.password(), &original_password);
    }

    #[test]
    fn send_and_recive() {
        let mut s = User::new(1, "abcd".to_string());
        s.add_friend(2);
        s.add_friend(3);
        s.add_invitation(10);
        s.add_invitation(11);

        let mut buffer = [0u8; crate::NET_BUFF_SIZE];
        s.try_into(&mut buffer).unwrap();
        let r = User::try_from(&buffer).unwrap();

        assert_eq!(s, r);
    }
}
