use std::sync::RwLock;

pub struct DbMock {
    _next_id: RwLock<i32>,
}

impl DbMock {
    pub fn new() -> Self {
        Self {
            _next_id: RwLock::new(1),
        }
    }

    pub fn insert_id(&self) -> i32 {

        let next_id = self._next_id.read().unwrap().clone();
        *self._next_id.write().unwrap() = next_id + 1;
        next_id
    }
}