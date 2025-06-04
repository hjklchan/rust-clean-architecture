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
    use chrono::Utc;

    use crate::entities::user::User;

    #[test]
    fn is_deleted_user_should_be_true() {
        let now_utc = Utc::now();

        let user = User {
            id: Some(1),
            username: "Lucas Chen".to_string(),
            email: "lucaschen@wtf.com".to_string(),
            hash_password: "hash_password".to_string(),
            status: Default::default(),
            gender: Default::default(),
            created_at: now_utc,
            updated_at: now_utc,
            deleted_at: Some(now_utc),
        };

        assert_eq!(user.is_deleted(), true);
    }

    #[test]
    fn is_deleted_user_should_be_false() {
        let now_utc = Utc::now();

        let user = User {
            id: Some(1),
            username: "Lucas Chen".to_string(),
            email: "lucaschen@wtf.com".to_string(),
            hash_password: "hash_password".to_string(),
            status: Default::default(),
            gender: Default::default(),
            created_at: now_utc,
            updated_at: now_utc,
            deleted_at: None,
        };

        assert_eq!(user.is_deleted(), true);
    }

    #[test]
    fn restore_user_should_work() {
        let now_utc = Utc::now();

        let mut user = User {
            id: Some(1),
            username: "Lucas Chen".to_string(),
            email: "lucaschen@wtf.com".to_string(),
            hash_password: "hash_password".to_string(),
            status: Default::default(),
            gender: Default::default(),
            created_at: now_utc,
            updated_at: now_utc,
            deleted_at: Some(now_utc),
        };

        user.restore();

        assert_eq!(user.is_deleted(), false);
    }

    #[test]
    fn mark_deleted_should_work() {
        let now_utc = Utc::now();

        let mut user = User {
            id: Some(1),
            username: "Lucas Chen".to_string(),
            email: "lucaschen@wtf.com".to_string(),
            hash_password: "hash_password".to_string(),
            status: Default::default(),
            gender: Default::default(),
            created_at: now_utc,
            updated_at: now_utc,
            deleted_at: Some(now_utc),
        };

        user.mark_deleted();

        assert_eq!(user.is_deleted(), true);
    }
}