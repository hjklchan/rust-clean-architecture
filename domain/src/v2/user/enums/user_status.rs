#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum UserStatus {
    #[default]
    Normal,
    Frozen,
    Disable,
}
