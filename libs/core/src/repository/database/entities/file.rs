use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "file")]
pub struct Model {
	#[sea_orm(primary_key, column_name = "file_id")]
	pub id: String,
	pub name: String,
	#[sea_orm(nullable)]
	pub alternative_text: Option<String>,
	#[sea_orm(nullable)]
	pub caption: Option<String>,
	#[sea_orm(nullable)]
	pub description: Option<String>,
	#[sea_orm(nullable)]
	pub mime_type: Option<String>,
	#[sea_orm(nullable)]
	pub size: Option<u64>,
	pub public_uri: Option<String>,
	#[sea_orm(nullable)]
	pub user_id: Option<String>,
	#[sea_orm(default_value = "A")]
	pub status_id: String,
	pub created_at: DateTimeUtc,
	pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub type File = Model;
