use chrono::{DateTime, Utc};

use crate::v2::user::{enums::user_status::UserStatus, value_objects::user_id::UserId};

// Aggregation root for user entity.
pub struct User {
    pub id: UserId,
    pub username: String,
    pub email: String,
    pub status: UserStatus,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(id: UserId, username: String, email: String, password: String) -> Self {
        let utc_now = Utc::now();

        Self {
            id,
            username,
            email,
            status: Default::default(),
            password,
            created_at: utc_now,
            updated_at: utc_now,
            deleted_at: None,
        }
    }
}
