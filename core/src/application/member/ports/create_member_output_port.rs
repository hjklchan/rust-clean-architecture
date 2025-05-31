pub struct CreateMemberOutputData {
    pub new_id: u64,
}

pub trait CreateMemberOutputPort: Send + Sync {
    fn present(&self, output_data: CreateMemberOutputData);
}
