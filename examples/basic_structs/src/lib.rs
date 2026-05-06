use ferryx_macros::ferryx;

#[ferryx]
pub struct UserProfile {
    pub id: u64,
    pub name: String,
    pub active: bool,
}

#[ferryx]
impl UserProfile {
    pub fn rename(&self, new_name: String) -> UserProfile {
        UserProfile {
            id: self.id,
            name: new_name,
            active: self.active,
        }
    }
}

