use chrono::{DateTime, Utc};

use crate::v2::user::{enums::user_status::UserStatus, value_objects::user_id::UserId};

// Aggregation root for user entity.
pub struct User {
    id: UserId,
    username: String,
    email: String,
    status: UserStatus,
    password: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
}
