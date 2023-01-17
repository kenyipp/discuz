pub use crate::repository::{
	database::db_post_comment::{
		CreateCommentInput, DbPostComment, DbPostCommentTrait, PostComment,
	},
	errors::RepoError,
};
use error_stack::{IntoReport, Result, ResultExt};

#[derive(Debug, Clone)]
pub struct RepoPostComment {
	db_post_comment: DbPostComment,
}

impl RepoPostComment {
	pub fn new(db_post_comment: DbPostComment) -> RepoPostComment {
		RepoPostComment { db_post_comment }
	}
}

#[async_trait]
pub trait RepoPostCategoryTrait {
	async fn find_by_id(&self, id: i32) -> Result<Option<PostComment>, RepoError>;
	async fn create(&self, input: &CreateCommentInput) -> Result<i32, RepoError>;
	async fn delete(&self, id: i32) -> Result<(), RepoError>;
}

#[async_trait]
impl RepoPostCategoryTrait for RepoPostComment {
	async fn find_by_id(&self, id: i32) -> Result<Option<PostComment>, RepoError> {
		self.db_post_comment
			.find_by_id(id)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn create(&self, input: &CreateCommentInput) -> Result<i32, RepoError> {
		self.db_post_comment
			.create(input)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn delete(&self, id: i32) -> Result<(), RepoError> {
		self.db_post_comment
			.delete(id)
			.await
			.into_report()
			.change_context(RepoError::Generic)?;

		Ok(())
	}
}
