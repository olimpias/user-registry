use crate::domain::user::repository;
use crate::domain::user::entity;
use crate::domain::user::error;
use crate::infrastructure::memorystore::memory_store::InmemoryRepository;

pub struct UserRepository {
    inmemory_store: InmemoryRepository<String, entity::User>,
}

impl UserRepository {
    pub fn new() -> Self {
        UserRepository { inmemory_store:  InmemoryRepository::new() }
    }
}

impl repository::UserRepository for UserRepository {
    fn create_user(&self, user: entity::User) -> entity::UserResult<()> {
        self.inmemory_store.add_record(user.id.to_string(), user);
        Ok(())
    }

    fn get_user_by_id(&self, id: uuid::Uuid) -> entity::UserResult<entity::User> {
        let result = self.inmemory_store.get_record(&id.to_string());
        match result {
            Some(v) => Ok(v),
            None => Err(Box::new(error::UserError::NotFound)),
        }
    }

    fn delete_user_by_id(&self, id: uuid::Uuid) -> entity::UserResult<()> {
        self.inmemory_store.delete_record(&id.to_string());
        Ok(())
    }

    fn get_all_users(&self) -> entity::UserResult<Vec<entity::User>> {
        let result = self.inmemory_store.get_records();
        Ok(result)
    }
}