use crate::entity::EntityManager;

pub struct World {
    entity_manager: EntityManager,
}

impl World {
    pub fn new(max_entity_count: usize) -> Self {
        Self {
            entity_manager: EntityManager::new(max_entity_count),
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new(usize::MAX)
    }
}
