use crate::entity::{Entity, EntityManager};

pub struct World {
    entity_manager: EntityManager,
}

impl World {
    pub fn new(max_entity_count: usize) -> Self {
        Self {
            entity_manager: EntityManager::new(max_entity_count),
        }
    }

    pub fn create_entity(&mut self) -> Option<Entity> {
        self.entity_manager.create_entity()
    }

    pub fn delete_entity(&mut self, entity: Entity) {
        // Safety: Due to the fact that entities cannot be cloned outside the
        // crate and this function takes ownership of the entity handle, it is
        // guaranteed that the entity was never deleted before.
        unsafe { self.entity_manager.delete_entity(entity) };
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new(usize::MAX)
    }
}
