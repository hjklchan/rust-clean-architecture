use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UserStatus {
    Normal,
    Frozen,
    Disabled,
}

impl From<UserStatus> for i8 {
    fn from(value: UserStatus) -> Self {
        match value {
            UserStatus::Normal => 0,
            UserStatus::Frozen => 1,
            UserStatus::Disabled => 2,
        }
    }
}

impl Into<UserStatus> for i8 {
    fn into(self) -> UserStatus {
        match self {
            0 => UserStatus::Normal,
            1 => UserStatus::Frozen,
            2 => UserStatus::Disabled,
            _ => UserStatus::Normal
        }
    }
}

pub struct User {
    pub id: Option<u64>,
    pub username: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub status: UserStatus,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(
        username: String,
        email: String,
        avatar_url: Option<String>,
        password_hash: String,
    ) -> Self {
        let utc_now = Utc::now();

        Self {
            id: None,
            username,
            email,
            avatar_url,
            status: UserStatus::Normal,
            password_hash,
            created_at: utc_now,
            updated_at: utc_now,
            deleted_at: None,
        }
    }
}

impl User {
    pub fn soft_delete(&mut self) {
        let utc_now = Utc::now();
        self.deleted_at = Some(utc_now);
        self.updated_at = utc_now;
    }

    pub fn restore(&mut self) {
        let utc_now = Utc::now();
        self.deleted_at = None;
        self.updated_at = utc_now;
    }
}