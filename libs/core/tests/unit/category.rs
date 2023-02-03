use std::sync::Arc;
use strum::EnumProperty;

use discuz_core::{
	constants::UNCLASSIFIED_CATEGORY_ID,
	migration::{Migrator, MigratorTrait},
	repository::{database::category::DbCategory, repo_category::RepoCategory},
	service::category::category_service::{
		CategoryService, CategoryServiceTrait, CreateCategoryInput, GetCategoriesResponse,
		UpdateCategoryInput,
	},
};
use discuz_utils::get_db_connection;

#[tokio::test]
async fn get_categories_basic() {
	let SetupResponse {
		category_service, ..
	} = setup().await;
	let GetCategoriesResponse { data, count } =
		category_service.get_categories(None).await.unwrap();
	assert!(data.len() > 1);
	assert!(count > 1);
}

#[tokio::test]
async fn get_unclassified_category() {
	let SetupResponse {
		category_service, ..
	} = setup().await;
	let category = category_service
		.find_by_id(UNCLASSIFIED_CATEGORY_ID)
		.await
		.unwrap();
	assert!(category.is_some());
	let category = category.unwrap();
	assert_eq!(category.name, "Chit chat");
	assert_eq!(category.slug, "chit-chat");
}

#[tokio::test]
async fn create_new_category() {
	let SetupResponse {
		category_service, ..
	} = setup().await;
	let category_name = "Hello world".to_owned();
	let input = CreateCategoryInput {
		name: category_name.to_owned(),
		description: None,
		level: 1,
		postable: false,
		user_id: None,
		parent_id: None,
		sort_index: None,
	};
	let category = category_service.create(&input).await.unwrap();
	assert_eq!(category.name, category_name);
}

#[tokio::test]
async fn create_duplicated_category() {
	let SetupResponse {
		category_service, ..
	} = setup().await;
	let category_name = "Hello world".to_owned();
	let input = CreateCategoryInput {
		name: category_name.to_owned(),
		description: None,
		level: 1,
		postable: false,
		user_id: None,
		parent_id: None,
		sort_index: None,
	};

	let category = category_service.create(&input).await;
	assert!(category.is_ok());

	let category = category_service.create(&input).await;
	assert!(category.is_err());

	let error = category
		.err()
		.unwrap()
		.current_context()
		.get_str("code")
		.unwrap();

	assert_eq!(error, "duplicate_category");
}

#[tokio::test]
async fn update_category() {
	let SetupResponse {
		category_service, ..
	} = setup().await;
	let category_name = "Hello world".to_owned();
	let category_description = "Foo Bar".to_owned();
	let input = UpdateCategoryInput {
		id: UNCLASSIFIED_CATEGORY_ID.to_owned(),
		name: category_name.to_owned(),
		description: Some(category_description.to_owned()),
		level: 1,
		postable: false,
		user_id: None,
		parent_id: None,
		status_id: None,
		sort_index: 100,
	};
	let category = category_service.update(&input).await.unwrap();
	assert_eq!(category.name, category_name);
	assert_eq!(category.description, Some(category_description));
}

#[tokio::test]
async fn update_not_exist_category() {
	let SetupResponse {
		category_service, ..
	} = setup().await;
	let category_name = "Hello world".to_owned();
	let category_description = "Foo Bar".to_owned();
	let input = UpdateCategoryInput {
		id: "NON_EXIST_CATEGORY_ID".to_owned(),
		name: category_name.to_owned(),
		description: Some(category_description.to_owned()),
		level: 1,
		postable: false,
		user_id: None,
		status_id: None,
		parent_id: None,
		sort_index: 100,
	};
	let category = category_service.update(&input).await;
	assert!(category.is_err());

	let error = category
		.err()
		.unwrap()
		.current_context()
		.get_str("code")
		.unwrap();

	assert_eq!(error, "category_not_exist");
}

#[tokio::test]
async fn delete_category() {
	let SetupResponse {
		category_service, ..
	} = setup().await;
	category_service
		.delete(UNCLASSIFIED_CATEGORY_ID)
		.await
		.unwrap();
	let category = category_service
		.find_by_id(UNCLASSIFIED_CATEGORY_ID)
		.await
		.unwrap()
		.unwrap();
	assert_eq!(category.status_id, "D");
}

#[tokio::test]
async fn delete_not_exist_category() {
	let SetupResponse {
		category_service, ..
	} = setup().await;
	let delete_category_response = category_service.delete("NON_EXIST_CATEGORY_ID").await;
	assert!(delete_category_response.is_err());

	let error = delete_category_response
		.err()
		.unwrap()
		.current_context()
		.get_str("code")
		.unwrap();

	assert_eq!(error, "category_not_exist");
}

#[tokio::test]
async fn create_category_after_deleted() {
	let SetupResponse {
		category_service, ..
	} = setup().await;

	category_service
		.delete(UNCLASSIFIED_CATEGORY_ID)
		.await
		.unwrap();

	let category = category_service
		.find_by_id(UNCLASSIFIED_CATEGORY_ID)
		.await
		.unwrap()
		.unwrap();

	assert_eq!(category.status_id, "D");

	let input = CreateCategoryInput {
		name: "Chit Chat".to_owned(),
		level: 1,
		postable: false,
		parent_id: None,
		description: None,
		user_id: None,
		sort_index: None,
	};

	let category = category_service.create(&input).await;

	assert!(category.is_ok());

	let category = category.unwrap();

	assert_eq!(category.id, UNCLASSIFIED_CATEGORY_ID);
	assert_eq!(category.status_id, "A");
}

async fn setup() -> SetupResponse {
	let db_connection = Arc::new(get_db_connection().await.unwrap());
	let category = DbCategory::new(&db_connection);
	let repo_category = RepoCategory::new(category);
	let category_service = Arc::new(CategoryService { repo_category });
	Migrator::refresh(&db_connection).await.unwrap();
	SetupResponse { category_service }
}

pub struct SetupResponse {
	category_service: Arc<CategoryService>,
}
