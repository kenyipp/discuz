use crate::{
	repository::repo_post::{Post, RepoPost, RepoPostTrait},
	service::post::errors::PostError,
};
use error_stack::Result;

pub async fn execute(repo_post: &RepoPost, id: i32) -> Result<Option<Post>, PostError> {
	let post = repo_post.find_by_id(id).await.map_err(|error| {
		println!("{error:#?}");
		PostError::InternalServerError
	})?;
	Ok(post)
}
