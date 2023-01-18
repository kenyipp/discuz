use crate::{
	repository::repo_post_comment::{PostComment, RepoPostCategoryTrait, RepoPostComment},
	service::post_comment::errors::PostCommentError,
};
use error_stack::Result;
use tracing::error;

pub async fn execute(
	repo_post_comment: &RepoPostComment,
	id: i32,
) -> Result<Option<PostComment>, PostCommentError> {
	let post_comment = repo_post_comment.find_by_id(id).await.map_err(|error| {
		error!("{:#?}", error);
		PostCommentError::InternalServerError
	})?;
	Ok(post_comment)
}
