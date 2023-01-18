use error_stack::Result;
use std::{fmt::Debug, sync::Arc};

pub use crate::repository::repo_post_reply::{CreateCommentInput, PostReply};

use crate::{
	repository::repo_post_reply::RepoPostReply,
	service::{
		post::post_service::PostServiceTrait,
		post_reply::{errors::PostReplyError, utils},
	},
};

#[derive(Debug, Clone)]
pub struct PostReplyService {
	pub post_service: Arc<dyn PostServiceTrait>,
	pub repo_post_reply: RepoPostReply,
}

#[async_trait]
pub trait PostReplyServiceTrait: Sync + Send + Debug {
	async fn find_by_id(&self, id: i32) -> Result<Option<PostReply>, PostReplyError>;
	async fn create(&self, input: &CreateCommentInput) -> Result<PostReply, PostReplyError>;
	async fn delete(&self, id: i32) -> Result<(), PostReplyError>;
}

#[async_trait]
impl PostReplyServiceTrait for PostReplyService {
	async fn find_by_id(&self, id: i32) -> Result<Option<PostReply>, PostReplyError> {
		utils::find_by_id::execute(&self.repo_post_reply, id).await
	}

	async fn create(&self, input: &CreateCommentInput) -> Result<PostReply, PostReplyError> {
		utils::create::execute(&self.repo_post_reply, &*self.post_service, input).await
	}

	async fn delete(&self, id: i32) -> Result<(), PostReplyError> {
		utils::delete::execute(&self.repo_post_reply, id).await
	}
}
