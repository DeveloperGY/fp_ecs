/// # Safety
/// Only ever use an entity with the world that created it. Using an entity with
/// a different world will probably give unintended behaviour
pub struct Entity(pub(crate) usize);

impl Entity {
    /// Cloning an entity can only be done within the ecs, otherwise the safety
    /// guarantees required for the entity manager are null and void
    pub(crate) fn clone(&self) -> Self {
        Entity(self.0)
    }
}

pub struct EntityManager {
    next_id: usize,
    max_entity_count: usize,
    deleted_id_queue: Vec<usize>,
}

impl EntityManager {
    pub fn new(max_entity_count: usize) -> Self {
        Self {
            next_id: 0,
            max_entity_count,
            deleted_id_queue: vec![],
        }
    }

    /// Creates a new entity.
    ///
    /// First checks to see if there are any dead ids that can be used, if so it
    /// uses one. If there arent any, then it will check the next available new
    /// id, and will use that if the maximum entity count hasnt been reached.
    /// Otherwise an entity cannot be created and this function returns None.
    pub fn create_entity(&mut self) -> Option<Entity> {
        match (self.deleted_id_queue.len(), self.next_id) {
            (1.., _) => Some(Entity(self.deleted_id_queue.pop().unwrap())),
            (0, new_id) if new_id < self.max_entity_count => {
                self.next_id += 1;
                Some(Entity(new_id))
            }
            _ => None,
        }
    }

    /// # Safety
    /// Once an entity is deleted, you must not delete it again until an entity
    /// with the deleted id is recreated by create_entity. Ignoring this will
    /// lead to the entity manager creating multiple entities with the same id.
    ///
    pub unsafe fn delete_entity(&mut self, entity: Entity) {
        self.deleted_id_queue.push(entity.0);
    }
}

impl Default for EntityManager {
    fn default() -> Self {
        Self::new(usize::MAX)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn perf_delete() {
        const ENTITY_COUNT: usize = 10_000_000;
        let mut em = EntityManager::default();

        let mut entities = Vec::with_capacity(ENTITY_COUNT);

        for _ in 0..ENTITY_COUNT {
            entities.push(em.create_entity().unwrap());
        }

        unsafe {
            for e in entities {
                em.delete_entity(e);
            }
        }
    }

    #[test]
    fn perf_recreate() {
        const ENTITY_COUNT: usize = 10_000_000;

        fn first_pass(em: &mut EntityManager) {
            let mut entities = Vec::with_capacity(ENTITY_COUNT);
            for _ in 0..ENTITY_COUNT {
                entities.push(em.create_entity().unwrap());
            }

            unsafe {
                for e in entities {
                    em.delete_entity(e);
                }
            }
        }

        fn second_pass(em: &mut EntityManager) {
            let mut entities = Vec::with_capacity(ENTITY_COUNT);
            for _ in 0..ENTITY_COUNT {
                entities.push(em.create_entity().unwrap());
            }
            unsafe {
                for e in entities {
                    em.delete_entity(e);
                }
            }
        }

        let mut em = EntityManager::default();
        first_pass(&mut em);
        second_pass(&mut em);
    }
}
