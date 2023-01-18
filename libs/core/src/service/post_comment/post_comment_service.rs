use error_stack::Result;
use std::fmt::Debug;

pub use crate::repository::repo_post_comment::{CreateCommentInput, PostComment};

use crate::{
	repository::repo_post_comment::RepoPostComment,
	service::post_comment::{errors::PostCommentError, utils},
};

#[derive(Debug, Clone)]
pub struct PostCommentService {
	pub repo_post_comment: RepoPostComment,
}

#[async_trait]
pub trait PostCommentServiceTrait: Sync + Send + Debug {
	async fn find_by_id(&self, id: i32) -> Result<Option<PostComment>, PostCommentError>;
	async fn create(&self, input: &CreateCommentInput) -> Result<PostComment, PostCommentError>;
	async fn delete(&self, id: i32) -> Result<(), PostCommentError>;
}

#[async_trait]
impl PostCommentServiceTrait for PostCommentService {
	async fn find_by_id(&self, id: i32) -> Result<Option<PostComment>, PostCommentError> {
		utils::find_by_id::execute(&self.repo_post_comment, id).await
	}

	async fn create(&self, input: &CreateCommentInput) -> Result<PostComment, PostCommentError> {
		utils::create::execute(&self.repo_post_comment, input).await
	}

	async fn delete(&self, id: i32) -> Result<(), PostCommentError> {
		utils::delete::execute(&self.repo_post_comment, id).await
	}
}
