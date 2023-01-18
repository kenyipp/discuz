use crate::{
	repository::repo_post_comment::{RepoPostCategoryTrait, RepoPostComment},
	service::post_comment::errors::PostCommentError,
};
use error_stack::Result;
use tracing::error;

pub async fn execute(repo_post_comment: &RepoPostComment, id: i32) -> Result<(), PostCommentError> {
	repo_post_comment.delete(id).await.map_err(|error| {
		error!("{:#?}", error);
		PostCommentError::InternalServerError
	})?;
	Ok(())
}
