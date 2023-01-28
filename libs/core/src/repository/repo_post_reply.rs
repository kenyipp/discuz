pub use crate::repository::{
	database::db_post_reply::{CreateCommentInput, DbPostReply, DbPostReplyTrait, PostReply},
	errors::RepoError,
};
use error_stack::{IntoReport, Result, ResultExt};

#[derive(Debug, Clone)]
pub struct RepoPostReply {
	db_post_reply: DbPostReply,
}

impl RepoPostReply {
	pub fn new(db_post_reply: DbPostReply) -> RepoPostReply {
		RepoPostReply { db_post_reply }
	}
}

#[async_trait]
pub trait RepoPostReplyTrait {
	async fn find_by_id(&self, id: i32) -> Result<Option<PostReply>, RepoError>;
	async fn create(&self, input: &CreateCommentInput) -> Result<i32, RepoError>;
	async fn delete(&self, id: i32) -> Result<(), RepoError>;
}

#[async_trait]
impl RepoPostReplyTrait for RepoPostReply {
	async fn find_by_id(&self, id: i32) -> Result<Option<PostReply>, RepoError> {
		self.db_post_reply
			.find_by_id(id)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn create(&self, input: &CreateCommentInput) -> Result<i32, RepoError> {
		self.db_post_reply
			.create(input)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn delete(&self, id: i32) -> Result<(), RepoError> {
		self.db_post_reply
			.delete(id)
			.await
			.into_report()
			.change_context(RepoError::Generic)?;
		Ok(())
	}
}
