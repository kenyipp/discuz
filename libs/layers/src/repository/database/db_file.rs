use std::sync::Arc;

use chrono;
use sea_orm::{DatabaseConnection, *};
use uuid::Uuid;

use super::entities::file;
pub use super::entities::file::File;

#[derive(Debug, Clone)]
pub struct DbFile {
	db_connection: Arc<DatabaseConnection>,
}

#[async_trait]
pub trait DbFileTrait {
	async fn create(&self, input: &CreateFileInput) -> Result<String, DbErr>;
	async fn update(&self, input: &UpdateFileInput) -> Result<(), DbErr>;
	async fn update_status(&self, id: &str, status: &str) -> Result<(), DbErr>;
	async fn find_by_id(&self, id: &str) -> Result<Option<File>, DbErr>;
}

impl DbFile {
	pub fn new(db_connection: &Arc<DatabaseConnection>) -> DbFile {
		DbFile {
			db_connection: db_connection.clone(),
		}
	}
}

#[async_trait]
impl DbFileTrait for DbFile {
	async fn create(&self, input: &CreateFileInput) -> Result<String, DbErr> {
		let file_id = Uuid::new_v4().to_string();
		let now = chrono::offset::Utc::now();
		let file = file::ActiveModel {
			id: Set(file_id.clone()),
			name: Set(input.name.to_owned()),
			alternative_text: Set(input.alternative_text.to_owned()),
			caption: Set(input.caption.to_owned()),
			description: Set(input.description.to_owned()),
			mime_type: Set(input.mime_type.to_owned()),
			size: Set(input.size.to_owned()),
			public_uri: Set(input.public_uri.to_owned()),
			user_id: Set(input.user_id.to_owned()),
			status_id: Set("A".to_owned()),
			created_at: Set(now),
			updated_at: Set(now),
		};
		file::Entity::insert(file)
			.exec(&*self.db_connection)
			.await?;
		Ok(file_id)
	}

	async fn find_by_id(&self, id: &str) -> Result<Option<File>, DbErr> {
		file::Entity::find()
			.filter(file::Column::Id.eq(id))
			.one(&*self.db_connection)
			.await
	}

	async fn update(&self, input: &UpdateFileInput) -> Result<(), DbErr> {
		let mut file: file::ActiveModel = self
			.find_by_id(&input.id)
			.await?
			.ok_or_else(|| DbErr::Custom(format!("File with id #{} not exist", input.id)))?
			.into();

		file.name = Set(input.name.to_owned());
		file.alternative_text = Set(input.alternative_text.to_owned());
		file.caption = Set(input.caption.to_owned());
		file.description = Set(input.description.to_owned());
		file.updated_at = Set(chrono::offset::Utc::now());

		file.update(&*self.db_connection).await?;
		Ok(())
	}

	async fn update_status(&self, id: &str, status_id: &str) -> Result<(), DbErr> {
		if status_id != "A" && status_id != "D" {
			return Err(DbErr::Custom("Invalid file status".to_string()));
		}

		let mut file: file::ActiveModel = self
			.find_by_id(id)
			.await?
			.ok_or_else(|| DbErr::Custom(format!("File with id #{} not exist", id)))?
			.into();

		file.status_id = Set(status_id.to_owned());

		file.update(&*self.db_connection).await?;
		Ok(())
	}
}

//

#[derive(Debug, Clone)]
pub struct CreateFileInput {
	pub name: String,
	pub alternative_text: Option<String>,
	pub caption: Option<String>,
	pub description: Option<String>,
	pub mime_type: Option<String>,
	pub size: Option<u64>,
	pub public_uri: Option<String>,
	pub user_id: Option<String>,
}

//

#[derive(Debug, Clone)]
pub struct UpdateFileInput {
	pub id: String,
	pub name: String,
	pub alternative_text: Option<String>,
	pub caption: Option<String>,
	pub description: Option<String>,
}
