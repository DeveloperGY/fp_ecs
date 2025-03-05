pub struct Entity(pub(crate) usize);

pub struct EntityManager {
    next_id: usize,
}

impl EntityManager {
    pub fn new() -> Self {
        Self { next_id: 0 }
    }

    pub fn create_entity(&mut self) -> Option<Entity> {
        match self.next_id {
            usize::MAX => None,
            id => Some(Entity(id)),
        }
    }
}
