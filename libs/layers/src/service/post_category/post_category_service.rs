use crate::{repository::repo_post_category::DefPostCategory, service::post_category::utils};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct PostCategoryService {
	pub repo_file: DefPostCategory,
}

#[async_trait]
pub trait PostCategoryServiceTrait: Sync + Send + Debug {}
