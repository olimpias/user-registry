use uuid::Uuid;
use std::error::Error;

pub type UserResult<T> = Result<T,Box<dyn Error>>;
#[derive(Clone, Debug)]
pub struct User {
    pub name: String,
    pub id: Uuid,
}