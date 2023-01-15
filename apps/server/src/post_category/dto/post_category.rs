use discuz_layers::repository::repo_post_category::DefPostCategory;
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Debug, Serialize, Deserialize)]
pub struct DtoPostCategory {
	pub id: String,
	pub name: String,
	pub slug: String,
	pub description: Option<String>,
	pub count: i64,
}

impl From<DefPostCategory> for DtoPostCategory {
	fn from(def_post_category: DefPostCategory) -> Self {
		Self {
			id: def_post_category.id,
			name: def_post_category.name,
			slug: def_post_category.slug,
			description: def_post_category.description,
			count: def_post_category.count,
		}
	}
}
