use std::collections::HashSet;

pub type UserID = u64;

#[derive(Debug)]
pub struct User {
    id: UserID,
    password: String,
    friends: HashSet<UserID>,
    invitations: HashSet<UserID>,
}

impl User {
    pub fn new(id: UserID, password: String) -> Self {
        Self {
            id,
            password,
            friends: HashSet::new(),
            invitations: HashSet::new(),
        }
    }

    pub fn id(&self) -> UserID {
        self.id
    }

    pub fn password(&self) -> &String {
        &self.password
    }

    pub fn change_password(&mut self, new_password: String, current_password: &String) -> bool {
        if self.password == *current_password {
            self.password = new_password;
            true
        } else {
            false
        }
    }

    pub fn friends(&self) -> &HashSet<UserID> {
        &self.friends
    }

    pub fn has_friend(&self, id: &UserID) -> bool {
        self.friends.contains(id)
    }

    pub fn add_friend(&mut self, id: UserID) -> bool {
        if !self.has_friend(&id) && self.has_invitation(&id) {
            self.invitations.remove(&id);
            self.friends.insert(id)
        } else {
            false
        }
    }

    pub fn remove_friend(&mut self, id: UserID) -> bool {
        self.friends.remove(&id)
    }

    pub fn invitations(&self) -> &HashSet<UserID> {
        &self.invitations
    }

    pub fn has_invitation(&self, id: &UserID) -> bool {
        self.invitations.contains(id)
    }

    pub fn add_invitation(&mut self, id: UserID) -> bool {
        if !self.has_invitation(&id) && !self.has_friend(&id) {
            self.invitations.insert(id)
        } else {
            false
        }
    }

    pub fn remove_invitation(&mut self, id: UserID) -> bool {
        self.invitations.remove(&id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_invitation() {
        let requester_id = 1;
        let mut user = User::new(0, "abcd".to_string());
        assert!(user.add_invitation(requester_id));
        assert!(!user.add_invitation(requester_id));

        // We cannot have same user on invitations and friends at the same time.
        user.add_friend(requester_id);
        assert!(!user.add_invitation(requester_id));
    }

    #[test]
    fn reject_invitation() {
        let requester_id = 1;
        let mut user = User::new(0, "abcd".to_string());
        assert!(!user.remove_invitation(requester_id));

        user.add_invitation(requester_id);
        assert!(user.remove_invitation(requester_id));
        assert!(!user.remove_invitation(requester_id));
    }

    #[test]
    fn add_friend() {
        let requester_id = 1;
        let mut user = User::new(0, "abcd".to_string());
        assert!(!user.add_friend(requester_id));
        user.add_invitation(requester_id);
        assert!(user.add_friend(requester_id));
        assert!(!user.add_friend(requester_id));
    }

    #[test]
    fn remove_friend() {
        let requester_id = 1;
        let mut user = User::new(0, "abcd".to_string());
        assert!(!user.remove_friend(requester_id));
        user.add_invitation(requester_id);
        assert!(!user.remove_friend(requester_id));
        user.add_friend(requester_id);
        assert!(user.remove_friend(requester_id));
        assert!(!user.remove_friend(requester_id));
    }

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
}
