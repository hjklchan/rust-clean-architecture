use chrono::prelude::{DateTime, Utc};

pub mod value_objects {
    #[allow(unused)]
    pub struct Address {
        pub province: String,
        pub city: String,
        pub street: String,
        pub full_address: Option<String>,
        pub postal_code: Option<String>,
    }
}

#[derive(Debug)]
pub struct Member {
    // identifier fields.
    pub id: Option<u64>,
    // business fields.
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub cellphone_number: String,
    pub address: Option<String>,
    pub postal_code: Option<String>,
    pub birthday: Option<DateTime<Utc>>,
    // the timestamp fields
    // The field 'created_at' will be determined by application.
    pub created_at: DateTime<Utc>,
    // The field 'updated_at' will be determined by application.
    pub updated_at: DateTime<Utc>,
    // The field 'deleted_at' is used to record deleting datetime.
    // Will be determined by application when the record is deleted.
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Member {
    pub fn new(
        first_name: String,
        last_name: String,
        email: String,
        cellphone_number: String,
        address: Option<String>,
        postal_code: Option<String>,
        birthday: Option<DateTime<Utc>>,
    ) -> Self {
        let now = Utc::now();

        Self {
            id: None,
            first_name,
            last_name,
            email,
            cellphone_number,
            address,
            postal_code,
            birthday,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        }
    }
}

impl Member {
    pub fn is_soft_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }

    pub fn can_forced_delete(&self) -> bool {
        !self.deleted_at.is_none()
    }

    pub fn fill_delete_at(&mut self) {
        let now = Utc::now();
        self.deleted_at = Some(now);
    }
}
