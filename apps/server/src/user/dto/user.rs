use chrono::{DateTime, Utc};
use discuz_core::repository::repo_user::User;
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Debug, Serialize, Deserialize)]
pub struct DtoUser {
	pub id: String,
	pub name: String,
	pub email: String,
	pub avatar_url: Option<String>,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

impl From<User> for DtoUser {
	fn from(user: User) -> Self {
		Self {
			id: user.id,
			name: user.name,
			email: user.email,
			avatar_url: user.avatar_url,
			created_at: user.created_at,
			updated_at: user.updated_at,
		}
	}
}
