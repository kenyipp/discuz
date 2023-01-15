use crate::repository::repo_post::RepoPost;
// use error_stack::Result;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct PostService {
	pub repo_post: RepoPost,
}
