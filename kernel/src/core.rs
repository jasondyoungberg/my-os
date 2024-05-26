pub struct CoreData {
    pub id: u32,
}

impl CoreData {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }
}
