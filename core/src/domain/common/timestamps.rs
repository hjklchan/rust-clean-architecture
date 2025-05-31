use chrono::{DateTime, Utc};

// The struct 'Timestamps' isn't used for time being.
#[allow(unused)]
pub struct Timestamps {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
