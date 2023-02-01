pub use crate::repository::{
	database::db_post::{CreatePostInput, DbPost, DbPostTrait, Post, UpdatePostInput},
	errors::RepoError,
};
use error_stack::{IntoReport, Result, ResultExt};

#[derive(Debug, Clone)]
pub struct RepoPost {
	db_post: DbPost,
}

impl RepoPost {
	pub fn new(db_post: DbPost) -> RepoPost {
		RepoPost { db_post }
	}
}

#[async_trait]
pub trait RepoPostTrait {
	// Post
	async fn find_by_id(&self, id: i32) -> Result<Option<Post>, RepoError>;
	async fn create(&self, input: &CreatePostInput) -> Result<i32, RepoError>;
	async fn update(&self, input: &UpdatePostInput) -> Result<(), RepoError>;
	async fn delete(&self, id: i32) -> Result<(), RepoError>;
}

#[async_trait]
impl RepoPostTrait for RepoPost {
	// Post
	async fn find_by_id(&self, id: i32) -> Result<Option<Post>, RepoError> {
		self.db_post
			.find_by_id(id)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn create(&self, input: &CreatePostInput) -> Result<i32, RepoError> {
		self.db_post
			.create(input)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn update(&self, input: &UpdatePostInput) -> Result<(), RepoError> {
		self.db_post
			.update(input)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn delete(&self, id: i32) -> Result<(), RepoError> {
		self.db_post
			.delete(id)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}
}
