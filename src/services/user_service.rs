use async_trait::async_trait;
use sqlx::Error;
use crate::models::user_model::UserModel;
use crate::repositories::user_repository::UserRepository;

#[async_trait]
pub trait UserService {
    async fn register_user(&self, user: UserModel) -> Result<UserModel, Error>;
    async fn login_user(&self, user: UserModel) -> Result<UserModel, Error>;
}

pub struct UserServiceImpl {
    repository: Box<dyn UserRepository>
}

impl UserService for UserServiceImpl {
    async fn register_user(&self, user: UserModel) -> Result<UserModel, Error> {
        // Disini dilakukan business logic, dimana validasi, proses data dilakukan disini sebelum masuk DB
        let register_user = &self.repository.insert_user(user);
        todo!()
    }

    async fn login_user(&self, user: UserModel) -> Result<UserModel, Error> {
        // Disini dilakukan validasi, hasilkan JWT Token
        let found_user = &self.repository.get_user(user);
        todo!()
    }
}