use super::entity;
use uuid::Uuid;

pub trait UserRepository {
    fn create_user(&self, user: entity::User) -> entity::UserResult<()>;
    fn get_user_by_id(&self, id: Uuid) -> entity::UserResult<entity::User>;
    fn delete_user_by_id(&self, id: Uuid) -> entity::UserResult<()>;
    fn get_all_users(&self) -> entity::UserResult<Vec<entity::User>>;
}