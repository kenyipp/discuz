pub use crate::repository::repo_post_comment::CreateCommentInput;
use crate::{
	repository::repo_post_comment::{PostComment, RepoPostCategoryTrait, RepoPostComment},
	service::post_comment::{errors::PostCommentError, utils::find_by_id},
};
use error_stack::{Result, ResultExt};

pub async fn execute(
	repo_post_comment: &RepoPostComment,
	input: &CreateCommentInput,
) -> Result<PostComment, PostCommentError> {
	let comment_id = repo_post_comment
		.create(input)
		.await
		.change_context(PostCommentError::InternalServerError)?;

	let post_comment = find_by_id::execute(repo_post_comment, comment_id)
		.await?
		.ok_or(PostCommentError::InternalServerError)?;

	Ok(post_comment)
}
