pub use crate::{
	repository::repo_post::Post,
	service::post::utils::{create::CreatePostInput, update::UpdatePostInput},
};
use crate::{
	repository::repo_post::RepoPost,
	service::post::{errors::PostError, utils},
};

use error_stack::Result;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct PostService {
	pub repo_post: RepoPost,
}

#[async_trait]
pub trait PostServiceTrait: Sync + Send + Debug {
	async fn find_by_id(&self, id: i32) -> Result<Option<Post>, PostError>;
	async fn create(&self, input: &CreatePostInput) -> Result<Post, PostError>;
	async fn update(&self, input: &UpdatePostInput) -> Result<Post, PostError>;
	async fn delete(&self, id: i32) -> Result<(), PostError>;
}

#[async_trait]
impl PostServiceTrait for PostService {
	async fn find_by_id(&self, id: i32) -> Result<Option<Post>, PostError> {
		utils::find_by_id::execute(&self.repo_post, id).await
	}

	async fn create(&self, input: &CreatePostInput) -> Result<Post, PostError> {
		utils::create::execute(&self.repo_post, input).await
	}

	async fn update(&self, input: &UpdatePostInput) -> Result<Post, PostError> {
		utils::update::execute(&self.repo_post, input).await
	}

	async fn delete(&self, id: i32) -> Result<(), PostError> {
		utils::delete::execute(&self.repo_post, id).await
	}
}
