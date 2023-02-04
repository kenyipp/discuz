use crate::{
	repository::repo_post::{self, Post, RepoPost, RepoPostTrait},
	service::post::{errors::PostError, utils::find_by_id},
};

use error_stack::{Result, ResultExt};
use slugify::slugify;

pub async fn execute(repo_post: &RepoPost, input: &UpdatePostInput) -> Result<Post, PostError> {
	let post = find_by_id::execute(repo_post, input.id)
		.await?
		.ok_or(PostError::PostNotExistError)?;

	let input = repo_post::UpdatePostInput {
		id: input.id.to_owned(),
		title: input.title.to_owned(),
		slug: slugify!(&input.title),
		category_id: input.category_id.to_owned(),
		max_comment_count: input.max_comment_count,
		content: input.content.to_owned(),
		status_id: input.status_id.to_owned().unwrap_or(post.status_id),
	};

	repo_post
		.update(&input)
		.await
		.change_context(PostError::InternalServerError)?;

	let post = find_by_id::execute(repo_post, input.id)
		.await?
		.ok_or(PostError::InternalServerError)?;
	Ok(post)
}

#[derive(Debug, Clone)]
pub struct UpdatePostInput {
	pub id: i32,
	pub title: String,
	pub category_id: String,
	pub max_comment_count: Option<i32>,
	pub content: String,
	pub status_id: Option<String>,
}
