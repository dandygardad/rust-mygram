use std::sync::Arc;
use async_trait::async_trait;
use crate::{models::user_model::UserModel, AppState};
use sqlx::Error;
use axum::extract::State;

#[async_trait]
pub trait UserRepository {
    async fn insert_user(&self, user: UserModel) -> Result<UserModel, Error>;
    async fn get_user(&self, user: UserModel) ->Result<UserModel, Error>;
}

pub struct UserRepositoryImpl {
    pub pool: State<Arc<AppState>>
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn insert_user(&self, user: UserModel) -> Result<UserModel, Error>{
        let insert_user = sqlx::query_as!(UserModel, "INSERT INTO users(username, email, password, age) VALUES ($1, $2, $3, $4) RETURNING id, username, email, password, age, created_at, updated_at", &user.username, &user.email, &user.password, &user.age).fetch_one(&self.pool).await?;

        Ok(insert_user)
    }

    async fn get_user(&self, user: UserModel) -> Result<UserModel, Error>{
        // Penggunaan await? dilakukan jika terjadi error maka akan return Err
        let get_user = sqlx::query_as!(UserModel,"SELECT id, username, email, password, age, created_at, updated_at FROM users WHERE id=$1", &user.id)
            .fetch_one(&self.pool)
            .await?;

        Ok(get_user)
    }
}