use crate::{
	repository::repo_post::RepoPost,
	service::post::{errors::PostError, utils},
};
pub use crate::{
	repository::repo_post::{Post, PostReply},
	service::post::utils::{
		create::CreatePostInput, create_reply::CreateReplyInput, update::UpdatePostInput,
	},
};

use error_stack::Result;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct PostService {
	pub repo_post: RepoPost,
}

#[async_trait]
pub trait PostServiceTrait: Sync + Send + Debug {
	// Post
	async fn find_by_id(&self, id: i32) -> Result<Option<Post>, PostError>;
	async fn create(&self, input: &CreatePostInput) -> Result<Post, PostError>;
	async fn update(&self, input: &UpdatePostInput) -> Result<Post, PostError>;
	async fn delete(&self, id: i32) -> Result<(), PostError>;
	// Reply
	async fn find_reply_by_id(&self, id: i32) -> Result<Option<PostReply>, PostError>;
	async fn create_reply(&self, input: &CreateReplyInput) -> Result<PostReply, PostError>;
	async fn delete_reply(&self, id: i32) -> Result<(), PostError>;
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

	async fn find_reply_by_id(&self, id: i32) -> Result<Option<PostReply>, PostError> {
		utils::find_reply_by_id::execute(&self.repo_post, id).await
	}

	async fn create_reply(&self, input: &CreateReplyInput) -> Result<PostReply, PostError> {
		utils::create_reply::execute(&self.repo_post, input).await
	}

	async fn delete_reply(&self, id: i32) -> Result<(), PostError> {
		utils::delete_reply::execute(&self.repo_post, id).await
	}
}
