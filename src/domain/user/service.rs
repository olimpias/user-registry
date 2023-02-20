use super::entity;
use super::repository;
use uuid::Uuid;
use std::sync::Arc;

pub struct Service {
    repository: Arc<dyn repository::UserRepository + Send + Sync>,
}


impl Service {
    pub fn new(repo: Arc<dyn repository::UserRepository + Send + Sync>) -> Self {
        Self { repository: repo}
    }

    #[tracing::instrument(
        name = "saving user into storage",
        skip(self, user)
    )]
    pub async fn create_user(&self, user: entity::User) -> entity::UserResult<entity::User> {
        let new_user = entity::User{name: user.name.clone(), id: Uuid::new_v4()};
        let response_new_user = new_user.clone();
        _ = self.repository.create_user(new_user);
        Ok(response_new_user)
    }

    pub async fn delete_user(&self, id: Uuid) -> entity::UserResult<()> {
        self.repository.delete_user_by_id(id)
    }

    pub async fn get_user(&self, id: Uuid) -> entity::UserResult<entity::User> {
        self.repository.get_user_by_id(id)
    }
    
    pub async fn get_users(&self) -> entity::UserResult<Vec<entity::User>> {
        self.repository.get_all_users()
    }
}

