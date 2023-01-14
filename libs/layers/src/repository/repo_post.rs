use derive_more::{Display, Error};
use error_stack::{IntoReport, Result, ResultExt};

pub use crate::repository::database::db_post::{
	CreatePostInput, CreatePostTagInput, DbPost, DbPostTrait, Post, PostTag, UploadPostInput,
	UploadPostTagInput,
};

#[derive(Debug, Clone)]
pub struct RepoPost {
	db_post: DbPost,
}

impl RepoPost {
	pub fn new(db_post: DbPost) -> RepoPost {
		RepoPost { db_post }
	}
}

#[derive(Debug, Error, Display)]
pub enum RepoError {
	#[display(fmt = "Repo Post Error: Generic")]
	Generic,
}
