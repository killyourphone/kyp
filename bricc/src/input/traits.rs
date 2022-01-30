pub enum UserInput {
    Number(u8),
    Star,
    Hash,
    Up,
    Down,
    SoftKey,
    Call,
    Power,
}

pub trait InputModule {
    fn get_input(&mut self) -> Option<UserInput>;
}
