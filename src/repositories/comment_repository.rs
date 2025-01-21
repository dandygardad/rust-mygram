use std::sync::Arc;
use axum::extract::State;
use sqlx::Error;
use crate::AppState;
use crate::models::comment_model::CommentModel;

pub trait CommentRepository {
    async fn get_all(&self) -> Result<Vec<CommentModel>, Error>;
    async fn get_one(&self, comment_id: i32) -> Result<CommentModel, Error>;
    async fn insert_comment(&self, comment: CommentModel) -> Result<bool, Error>;
    async fn update_comment(&self, comment: CommentModel) -> Result<bool, Error>;
    async fn delete_comment(&self, comment_id: i32) -> Result<bool, Error>;
}

pub struct CommentRepositoryImpl {
    pool: State<Arc<AppState>>
}

impl CommentRepository for CommentRepositoryImpl {
    async fn get_all(&self) -> Result<Vec<CommentModel>, Error>{
        let get_all_comments = sqlx::query_as!(CommentModel, "SELECT id, user_id, photo_id, message, created_at, updated_at FROM comments").fetch_all(&self.pool).await?;

        Ok(get_all_comments)
    }

    async fn get_one(&self, comment_id: i32) -> Result<CommentModel, Error> {
        let get_one_comment = sqlx::query_as!(CommentModel, "SELECT id, user_id, photo_id, message, created_at, updated_at FROM comments WHERE id=$1", comment_id).fetch_one(&self.pool).await?;

        Ok(get_one_comment)
    }

    async fn insert_comment(&self, comment: CommentModel) -> Result<bool, Error> {
        let insert_new_comment = sqlx::query_as!(CommentModel, "INSERT INTO comments(user_id, photo_id, message) VALUES ($1, $2, $3)", comment.user_id, comment.photo_id, comment.message).execute(&self.pool).await?;

        Ok(true)
    }

    async fn update_comment(&self, comment: CommentModel) -> Result<bool, Error> {
        let update_comment = sqlx::query_as!(CommentModel, "UPDATE comments SET user_id=$1, photo_id=$2, message=$3, updated_at=CURRENT_TIMESTAMP WHERE id=$4", comment.user_id, comment.photo_id, comment.message, comment.id).execute(&self.pool).await?;

        Ok(true)
    }

    async fn delete_comment(&self, comment_id: i32) -> Result<bool, Error> {
        let update_comment = sqlx::query_as!(CommentModel, "DELETE FROM comments where id=$1", comment_id).execute(&self.pool).await?;

        Ok(true)
    }
}