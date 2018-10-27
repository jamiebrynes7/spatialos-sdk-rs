#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct EntityId {
    pub id: i64,
}

impl EntityId {
    pub fn new(id: i64) -> EntityId {
        EntityId { id }
    }

    pub fn is_valid(&self) -> bool {
        self.id > 0
    }

    pub fn to_string(&self) -> String {
        format!("EntityId: {}", self.id)
    }
}
