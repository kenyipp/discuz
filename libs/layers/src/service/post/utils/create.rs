use crate::{
	repository::repo_post::{self, Post, RepoPost, RepoPostTrait},
	service::post::{errors::PostError, utils::find_by_id::execute as find_by_id},
};
use chrono;
use error_stack::{Result, ResultExt};
use slugify::slugify;

pub async fn execute(repo_post: &RepoPost, input: &CreatePostInput) -> Result<Post, PostError> {
	let time = chrono::offset::Utc::now();
	let slug = slugify!(&format!("{} {:?}", &input.title, time));

	let CreatePostInput {
		title,
		post_category_id,
		content,
		excerpt,
		user_id,
	} = input;

	let input = repo_post::CreatePostInput {
		title: title.to_owned(),
		slug,
		post_category_id: post_category_id.to_owned(),
		content: content.to_owned(),
		excerpt: excerpt.to_owned(),
		user_id: user_id.to_owned(),
	};

	let post_id = repo_post
		.create(&input)
		.await
		.change_context(PostError::InternalServerError)?;
	let post = find_by_id(repo_post, post_id)
		.await?
		.ok_or_else(|| PostError::InternalServerError)?;

	Ok(post)
}

#[derive(Debug, Clone)]
pub struct CreatePostInput {
	pub title: String,
	pub post_category_id: String,
	pub content: String,
	pub excerpt: String,
	pub user_id: Option<String>,
}
