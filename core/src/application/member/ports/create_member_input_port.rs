use chrono::{DateTime, Utc};

use crate::domain::member::Member;

pub struct CreateMemberInputData {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub cellphone_number: String,
    pub address: Option<String>,
    pub postal_code: Option<String>,
    pub birthday: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<CreateMemberInputData> for Member {
    fn from(value: CreateMemberInputData) -> Self {
        Self {
            id: None,
            first_name: value.first_name,
            last_name: value.last_name,
            email: value.email,
            cellphone_number: value.cellphone_number,
            address: value.address,
            postal_code: value.postal_code,
            birthday: value.birthday,
            created_at: value.created_at,
            updated_at: value.updated_at,
            deleted_at: None,
        }
    }
}

#[async_trait::async_trait]
pub trait CreateMemberInputPort {
    async fn create(&self, data: CreateMemberInputData);
}
