use std::convert::From;

use chrono::{DateTime, Utc};
use discuz_layers::repository::repo_user::User;
use serde::Serialize;

#[derive(Debug, Serialize)]
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
