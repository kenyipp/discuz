use crate::{
	repository::repo_post_reply::{RepoPostCategoryTrait, RepoPostReply},
	service::post_reply::errors::PostReplyError,
};
use error_stack::Result;
use tracing::error;

pub async fn execute(repo_post_reply: &RepoPostReply, id: i32) -> Result<(), PostReplyError> {
	repo_post_reply.delete(id).await.map_err(|error| {
		error!("{:#?}", error);
		PostReplyError::InternalServerError
	})?;
	Ok(())
}
