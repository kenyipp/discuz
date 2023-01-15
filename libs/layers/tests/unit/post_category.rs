use discuz_layers::{
	constants::UNCLASSIFIED_CATEGORY_ID,
	migration::{Migrator, MigratorTrait},
	repository::{
		database::db_post_category::DbPostCategory, repo_post_category::RepoPostCategory,
	},
	service::post_category::post_category_service::{
		CreateCategoryInput, PostCategoryService, PostCategoryServiceTrait, UpdateCategoryInput,
	},
};
use discuz_utils::get_db_connection;
use std::sync::Arc;

#[tokio::test]
async fn get_unclassified_category() {
	let SetupResponse {
		post_category_service,
		..
	} = setup().await;
	let post_category = post_category_service
		.find_by_id(UNCLASSIFIED_CATEGORY_ID)
		.await
		.unwrap();
	assert!(post_category.is_some());
	let post_category = post_category.unwrap();
	assert_eq!(post_category.name, "Unclassified");
	assert_eq!(post_category.slug, "unclassified");
}

#[tokio::test]
async fn create_new_category() {
	let SetupResponse {
		post_category_service,
		..
	} = setup().await;
	let category_name = "Hello world".to_owned();
	let input = CreateCategoryInput {
		name: category_name.to_owned(),
		description: None,
		user_id: None,
	};
	let post_category = post_category_service.create(&input).await.unwrap();
	assert_eq!(post_category.name, category_name);
}

#[tokio::test]
async fn update_category() {
	let SetupResponse {
		post_category_service,
		..
	} = setup().await;
	let category_name = "Hello world".to_owned();
	let category_description = "Foo Bar".to_owned();
	let input = UpdateCategoryInput {
		id: UNCLASSIFIED_CATEGORY_ID.to_owned(),
		name: category_name.to_owned(),
		description: Some(category_description.to_owned()),
		user_id: None,
	};
	let post_category = post_category_service.update(&input).await.unwrap();
	assert_eq!(post_category.name, category_name);
	assert_eq!(post_category.description, Some(category_description));
}

#[tokio::test]
async fn delete_category() {
	let SetupResponse {
		post_category_service,
		..
	} = setup().await;
	post_category_service
		.delete(UNCLASSIFIED_CATEGORY_ID)
		.await
		.unwrap();
	let post_category = post_category_service
		.find_by_id(UNCLASSIFIED_CATEGORY_ID)
		.await
		.unwrap();
	assert!(post_category.is_none());
}

async fn setup() -> SetupResponse {
	let db_connection = Arc::new(get_db_connection().await.unwrap());
	let db_post_category = DbPostCategory::new(&db_connection);
	let repo_post_category = RepoPostCategory::new(db_post_category);
	let post_category_service = Arc::new(PostCategoryService { repo_post_category });
	Migrator::refresh(&db_connection).await.unwrap();
	SetupResponse {
		post_category_service,
	}
}

pub struct SetupResponse {
	post_category_service: Arc<PostCategoryService>,
}
