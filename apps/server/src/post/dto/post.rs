use chrono::{DateTime, Utc};
use discuz_core::repository::repo_post::Post;
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Debug, Serialize, Deserialize)]
pub struct DtoCategory {
	pub id: i32,
	pub title: String,
	pub slug: String,
	pub category_id: String,
	pub content: String,
	pub user_id: Option<String>,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

impl From<Post> for DtoCategory {
	fn from(post: Post) -> Self {
		Self {
			id: post.id.to_owned(),
			title: post.title.to_owned(),
			slug: post.slug.to_owned(),
			category_id: post.category_id.to_owned(),
			content: post.content.to_owned(),
			user_id: post.user_id.to_owned(),
			created_at: post.created_at.to_owned(),
			updated_at: post.updated_at.to_owned(),
		}
	}
}
