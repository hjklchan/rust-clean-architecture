use chrono::{DateTime, Utc};

#[derive(Debug, Default)]
pub enum Status {
    #[default]
    Normal,
    Frozen,
    Blocked,
}

#[derive(Debug, Default)]
pub enum Gender {
    #[default]
    Unknown,
    Private,
    Male,   // 🚹
    Female, // 🚺
}

pub struct User {
    pub id: Option<u64>,
    pub username: String,
    pub email: String,
    pub hash_password: String,
    pub status: Status, // Default Status::Unknown
    pub gender: Gender,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

// Read
impl User {
    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }
}

// Write
impl User {
    pub fn restore(&mut self) {
        self.deleted_at = None;
    }

    pub fn mark_deleted(&mut self) {
        self.deleted_at = Some(Utc::now());
    }
}

#[cfg(test)]
mod tests {
    // TODO
    fn is_deleted_should_work() {}
}