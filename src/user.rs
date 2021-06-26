use std::collections::HashSet;

pub type UserID = u64;

pub struct User {
    id: UserID,
    friends: HashSet<UserID>,
    invitations: HashSet<UserID>,
}

impl User {
    pub fn new(id: UserID) -> Self {
        Self {
            id,
            friends: HashSet::new(),
            invitations: HashSet::new(),
        }
    }

    pub fn id(&self) -> UserID {
        self.id
    }

    pub fn friends(&self) -> &HashSet<UserID> {
        &self.friends
    }

    pub fn has_friend(&self, id: &UserID) -> bool {
        self.friends.contains(id)
    }

    pub fn add_friend(&mut self, id: UserID) -> bool {
        if !self.has_friend(&id) {
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
        if !self.has_invitation(&id) {
            self.invitations.insert(id)
        } else {
            false
        }
    }

    pub fn reject_invitation(&mut self, id: UserID) -> bool {
        self.invitations.remove(&id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_invitation() {
        let requester_id = 1;
        let mut user = User::new(0);
        assert!(user.add_invitation(requester_id));
        assert!(!user.add_invitation(requester_id));
    }

    #[test]
    fn reject_invitation() {
        let requester_id = 1;
        let mut user = User::new(0);
        user.add_invitation(requester_id);
        assert!(user.reject_invitation(requester_id));
        assert!(!user.reject_invitation(requester_id));
    }

    #[test]
    fn add_friend() {
        let requester_id = 1;
        let mut user = User::new(0);
        user.add_invitation(requester_id);
        assert!(user.add_friend(requester_id));
        assert!(!user.add_friend(requester_id));
    }

    #[test]
    fn remove_friend() {
        let requester_id = 1;
        let mut user = User::new(0);
        user.add_invitation(requester_id);
        user.add_friend(requester_id);
        assert!(user.remove_friend(requester_id));
        assert!(!user.remove_friend(requester_id));
    }
}
