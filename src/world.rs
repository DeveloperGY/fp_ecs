use crate::entity::EntityManager;

pub struct World {
    entity_manager: EntityManager,
}

impl World {
    pub fn new() -> Self {
        Self {
            entity_manager: EntityManager::new(),
        }
    }
}
