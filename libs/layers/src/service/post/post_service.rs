use crate::{
	repository::repo_post::{RepoPost, RepoPostTrait},
	service::post::{errors::PostError, utils},
};
use error_stack::Result;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct PostService {
	pub repo_post: RepoPost,
}
