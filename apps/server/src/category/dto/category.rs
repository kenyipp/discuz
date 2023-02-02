use discuz_core::repository::repo_category::Category;
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Debug, Serialize, Deserialize)]
pub struct DtoCategory {
	pub id: String,
	pub name: String,
	pub slug: String,
	pub description: Option<String>,
	pub count: i64,
}

impl From<Category> for DtoCategory {
	fn from(def_category: Category) -> Self {
		Self {
			id: def_category.id,
			name: def_category.name,
			slug: def_category.slug,
			description: def_category.description,
			count: def_category.count,
		}
	}
}
