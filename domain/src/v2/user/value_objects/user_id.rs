use std::fmt::Display;
use uuid::Uuid;

pub struct UserId(Uuid);

impl UserId {
    // Generate a random UUID.
    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn get_id(&self) -> Uuid {
        self.0
    }
}

impl AsRef<Uuid> for UserId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for UserId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}