use crate::{
	repository::repo_post_reply::{PostReply, RepoPostCategoryTrait, RepoPostReply},
	service::post_reply::errors::PostReplyError,
};
use error_stack::Result;
use tracing::error;

pub async fn execute(
	repo_post_reply: &RepoPostReply,
	id: i32,
) -> Result<Option<PostReply>, PostReplyError> {
	let post_reply = repo_post_reply.find_by_id(id).await.map_err(|error| {
		error!("{:#?}", error);
		PostReplyError::InternalServerError
	})?;
	Ok(post_reply)
}
