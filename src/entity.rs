pub struct Entity(pub(crate) usize);

pub struct EntityManager {
    next_id: usize,
    max_entity_count: usize,
}

impl EntityManager {
    pub fn new(max_entity_count: usize) -> Self {
        Self {
            next_id: 0,
            max_entity_count,
        }
    }

    pub fn create_entity(&mut self) -> Option<Entity> {
        match self.next_id {
            id if id < self.max_entity_count => Some(Entity(id)),
            _ => None,
        }
    }
}

impl Default for EntityManager {
    fn default() -> Self {
        Self::new(usize::MAX)
    }
}
