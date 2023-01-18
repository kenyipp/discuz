pub use crate::repository::{
	database::db_post::{
		CreatePostInput, CreatePostTagInput, DbPost, DbPostTrait, DefPostTag, Post,
		UpdatePostInput, UpdatePostTagInput,
	},
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
	// Def Post Tag
	// async fn find_post_tag_by_id(&self, id: &str) -> Result<Option<DefPostTag>, RepoError>;
	// async fn create_post_tag(&self, input: &CreatePostTagInput) -> Result<String, RepoError>;
	// async fn update_post_tag(&self, input: &UpdatePostTagInput) -> Result<(), RepoError>;
	//
	// async fn find_post_tags_by_post_id(&self, id: &str) -> Result<Vec<PostTag>, RepoError>;
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

	// Def Post Tag
	// async fn find_post_tag_by_id(&self, id: &str) -> Result<Option<DefPostTag>, RepoError> {
	// 	self.db_post
	// 		.find_post_tag_by_id(id)
	// 		.await
	// 		.into_report()
	// 		.change_context(RepoError::Generic)
	// }

	// async fn create_post_tag(&self, input: &CreatePostTagInput) -> Result<String, RepoError> {
	// 	self.db_post
	// 		.create_post_tag(input)
	// 		.await
	// 		.into_report()
	// 		.change_context(RepoError::Generic)
	// }

	// async fn update_post_tag(&self, input: &UpdatePostTagInput) -> Result<(), RepoError> {
	// 	self.db_post
	// 		.update_post_tag(input)
	// 		.await
	// 		.into_report()
	// 		.change_context(RepoError::Generic)
	// }

	// async fn find_post_tags_by_post_id(&self, id: &str) -> Result<Vec<PostTag>, RepoError> {
	// 	self.db_post
	// 		.find_post_tags_by_post_id(id)
	// 		.await
	// 		.into_report()
	// 		.change_context(RepoError::Generic)
	// }
}
