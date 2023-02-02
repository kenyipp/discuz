use std::sync::Arc;

use discuz_core::{
	constants::UNCLASSIFIED_CATEGORY_ID,
	migration::{Migrator, MigratorTrait},
	repository::{
		database::{category::DbCategory, db_post::DbPost},
		repo_category::RepoCategory,
		repo_post::RepoPost,
	},
	service::{
		category::category_service::{CategoryService, CategoryServiceTrait},
		post::post_service::{CreatePostInput, PostService, PostServiceTrait, UpdatePostInput},
	},
};
use discuz_utils::get_db_connection;

#[tokio::test]
async fn create_post() {
	let SetupResponse {
		post_service,
		category_service,
		..
	} = setup().await;

	let input = get_create_post_input();

	let post = post_service.create(&input).await.unwrap();
	assert!(post.slug.contains("hello-world"));
	assert_eq!(post.category_id, UNCLASSIFIED_CATEGORY_ID);

	let category = category_service
		.find_by_id(UNCLASSIFIED_CATEGORY_ID)
		.await
		.unwrap()
		.unwrap();
	assert_eq!(category.count, 1);
}

#[tokio::test]
async fn create_posts_with_same_content() {
	let SetupResponse {
		post_service,
		category_service,
		..
	} = setup().await;

	let input = get_create_post_input();

	post_service.create(&input).await.unwrap();
	post_service.create(&input).await.unwrap();

	let category = category_service
		.find_by_id(UNCLASSIFIED_CATEGORY_ID)
		.await
		.unwrap()
		.unwrap();
	assert_eq!(category.count, 2);
}

#[tokio::test]
async fn update_post() {
	let SetupResponse { post_service, .. } = setup().await;

	let input = get_create_post_input();

	let post = post_service.create(&input).await.unwrap();

	let update_input = UpdatePostInput {
		id: post.id,
		title: "Foo Bar".to_owned(),
		category_id: UNCLASSIFIED_CATEGORY_ID.to_owned(),
		content: "Content".to_owned(),
		max_comment_count: None,
		user_id: None,
		status_id: None,
	};

	let post = post_service.update(&update_input).await.unwrap();

	assert_eq!(post.title, "Foo Bar");
}

#[tokio::test]
async fn delete_post() {
	let SetupResponse {
		post_service,
		category_service,
		..
	} = setup().await;

	let input = get_create_post_input();

	let post = post_service.create(&input).await.unwrap();

	post_service.delete(post.id).await.unwrap();

	let post = post_service.find_by_id(post.id).await.unwrap().unwrap();

	assert_eq!(post.status_id, "D");

	let category = category_service
		.find_by_id(UNCLASSIFIED_CATEGORY_ID)
		.await
		.unwrap()
		.unwrap();

	assert_eq!(category.count, 0);
}

fn get_create_post_input() -> CreatePostInput {
	CreatePostInput {
		title: "Hello world".to_owned(),
		category_id: UNCLASSIFIED_CATEGORY_ID.to_owned(),
		content: "Content".to_owned(),
		user_id: None,
	}
}

async fn setup() -> SetupResponse {
	let db_connection = Arc::new(get_db_connection().await.unwrap());
	let category = DbCategory::new(&db_connection);
	let repo_category = RepoCategory::new(category);
	let category_service = Arc::new(CategoryService { repo_category });
	let db_post = DbPost::new(&db_connection);
	let repo_post = RepoPost::new(db_post);
	let post_service = Arc::new(PostService { repo_post });
	Migrator::refresh(&db_connection).await.unwrap();
	SetupResponse {
		post_service,
		category_service,
	}
}

pub struct SetupResponse {
	post_service: Arc<PostService>,
	category_service: Arc<CategoryService>,
}
