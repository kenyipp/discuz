use std::sync::Arc;

use discuz_core::{
	migration::{Migrator, MigratorTrait},
	repository::repo_file::{DbFile, RepoFile},
	service::{file::constants::FileType, prelude::*},
	utils::mock_data::mock_s3_provider::MockApiS3,
};
use discuz_utils::get_db_connection;

#[tokio::test]
async fn delete_file() {
	let SetupResponse { file_service } = setup().await;
	let file = file_service
		.get_upload_url(&FileType::Attachment)
		.await
		.unwrap();
	assert_eq!(file.upload_uri, "UPLOAD_URI");
	assert_eq!(file.public_uri, Some("PUBLIC_URI".to_owned()));
}

async fn setup() -> SetupResponse {
	let db_connection = Arc::new(get_db_connection().await.unwrap());
	let api_provider = Arc::new(MockApiS3);
	let db_file = DbFile::new(&db_connection.clone());
	let repo_file = RepoFile::new(db_file);
	let file_service = FileService {
		api_provider,
		repo_file,
	};
	Migrator::refresh(&db_connection).await.unwrap();
	SetupResponse {
		file_service: Arc::new(file_service),
	}
}

pub struct SetupResponse {
	file_service: Arc<FileService>,
}
