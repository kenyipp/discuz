use serde::{ Serialize, Deserialize };
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
	#[sea_orm(primary_key, column_name = "user_id")]
	pub id: String,
	#[sea_orm(unique)]
	pub sub: String,
	pub name: String,
	#[sea_orm(unique)]
	pub email: String,
	#[sea_orm(nullable)]
	pub avatar_url: Option<String>,
	#[sea_orm(nullable)]
	pub notes: Option<String>,
	#[sea_orm(default_value = "A")]
	pub status_id: String,
	pub created_at: DateTimeUtc,
	pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub type User = Model;
