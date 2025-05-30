use chrono::prelude::{DateTime, Utc};
use value_objects::{Content, Status};

pub mod value_objects {
    // [value_object] Content
    pub struct Content(String);

    impl Content {
        pub fn length(&self) -> usize {
            self.0.len()
        }
    }

    // [value_object] Status
    #[derive(Default)]
    pub enum Status {
        #[default]
        Draft,
        Pending,
        Published,
    }
}

pub struct Article {
    pub id: Option<u64>,
    pub title: String,
    pub description: Option<String>,
    pub content: Content,
    pub tags: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Article {
    pub fn new(
        title: String,
        description: Option<String>,
        content: Content,
        tags: Option<Vec<String>>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id: None,
            title,
            description,
            content,
            tags,
            created_at,
            updated_at,
            deleted_at: None,
        }
    }
}
