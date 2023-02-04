use std::sync::Arc;

use discuz_core::{
	constants::{FAKE_ACCESS_TOKEN, UNCLASSIFIED_CATEGORY_ID},
	migration::{Migrator, MigratorTrait},
	service::prelude::*,
};
use discuz_utils::{amazon::get_aws_sdk_config, get_db_connection};

#[tokio::test]
async fn create_post() {
	let SetupResponse {
		post_service,
		category_service,
		create_post_input,
		..
	} = setup().await;

	let post = post_service.create(&create_post_input).await.unwrap();
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
		create_post_input,
		..
	} = setup().await;

	post_service.create(&create_post_input).await.unwrap();
	post_service.create(&create_post_input).await.unwrap();

	let category = category_service
		.find_by_id(UNCLASSIFIED_CATEGORY_ID)
		.await
		.unwrap()
		.unwrap();
	assert_eq!(category.count, 2);
}

#[tokio::test]
async fn update_post() {
	let SetupResponse {
		post_service,
		create_post_input,
		..
	} = setup().await;

	let post = post_service.create(&create_post_input).await.unwrap();

	let update_input = UpdatePostInput {
		id: post.id,
		title: "Foo Bar".to_owned(),
		category_id: UNCLASSIFIED_CATEGORY_ID.to_owned(),
		content: "Content".to_owned(),
		max_comment_count: None,
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
		create_post_input,
		..
	} = setup().await;

	let post = post_service.create(&create_post_input).await.unwrap();

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

async fn setup() -> SetupResponse {
	let db_connection = Arc::new(get_db_connection().await.unwrap());
	let sdk_config = Arc::new(get_aws_sdk_config().await);
	Migrator::refresh(&db_connection).await.unwrap();
	let factory = Factory::new(&db_connection, &sdk_config);
	let category_service = Arc::new(factory.new_category_service());
	let post_service = Arc::new(factory.new_post_service());
	let auth_service = Arc::new(factory.new_auth_service());
	let user_service = Arc::new(factory.new_user_service(auth_service));
	let user = user_service.get_profile(FAKE_ACCESS_TOKEN).await.unwrap();
	let create_post_input = CreatePostInput {
		title: "Hello world".to_owned(),
		category_id: UNCLASSIFIED_CATEGORY_ID.to_owned(),
		content: "Content".to_owned(),
		user_id: user.id,
	};
	SetupResponse {
		post_service,
		category_service,
		create_post_input,
	}
}

pub struct SetupResponse {
	post_service: Arc<PostService>,
	category_service: Arc<CategoryService>,
	create_post_input: CreatePostInput,
}
