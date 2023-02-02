use std::sync::Arc;
use strum::EnumProperty;

use discuz_core::{
	constants::{FAKE_ACCESS_TOKEN, UNCLASSIFIED_CATEGORY_ID},
	migration::{Migrator, MigratorTrait},
	repository::{repo_post::Post, repo_user::User},
	service::{post::errors::PostError, prelude::*},
	utils::mock_data,
};
use discuz_utils::{amazon::get_aws_sdk_config, get_db_connection};

#[tokio::test]
async fn create_post_reply() {
	let SetupResponse {
		post,
		user,
		post_service,
		..
	} = setup().await;

	let input = CreateReplyInput {
		post_id: post.id.to_owned(),
		content: mock_data::post::COMMENT_CONTENT.to_owned(),
		quote_reply_id: None,
		user_id: user.id.to_owned(),
	};

	let post_reply = post_service.create_reply(&input).await;
	assert!(post_reply.is_ok());

	let post_reply = post_reply.unwrap();
	assert_eq!(
		post_reply.content,
		mock_data::post::COMMENT_CONTENT.to_owned()
	);

	let post = post_service.find_by_id(post.id).await.unwrap().unwrap();
	assert_eq!(post.comment_count, 1);
}

#[tokio::test]
async fn delete_post_reply() {
	let SetupResponse {
		post,
		user,
		post_service,
		..
	} = setup().await;

	let input = CreateReplyInput {
		post_id: post.id.to_owned(),
		content: mock_data::post::COMMENT_CONTENT.to_owned(),
		quote_reply_id: None,
		user_id: user.id.to_owned(),
	};

	let post_reply = post_service.create_reply(&input).await.unwrap();
	post_service.delete_reply(post_reply.id).await.unwrap();

	let post_reply = post_service
		.find_reply_by_id(post_reply.id)
		.await
		.unwrap()
		.unwrap();
	assert_eq!(post_reply.status_id, "D");

	let post = post_service.find_by_id(post.id).await.unwrap().unwrap();
	assert_eq!(post.comment_count, 1);
}

#[tokio::test]
async fn maximum_post_reply_error() {
	let SetupResponse {
		post,
		user,
		post_service,
		..
	} = setup().await;

	let input = UpdatePostInput {
		id: post.id,
		max_comment_count: Some(1),
		title: mock_data::post::POST_TITLE.to_owned(),
		category_id: UNCLASSIFIED_CATEGORY_ID.to_owned(),
		content: mock_data::post::POST_CONTENT.to_owned(),
		status_id: None,
		user_id: None,
	};

	post_service.update(&input).await.unwrap();

	let input = CreateReplyInput {
		post_id: post.id.to_owned(),
		content: mock_data::post::COMMENT_CONTENT.to_owned(),
		quote_reply_id: None,
		user_id: user.id.to_owned(),
	};

	let post_reply_resp = post_service.create_reply(&input).await;
	assert!(post_reply_resp.is_ok());

	let post_reply_resp = post_service.create_reply(&input).await;
	assert!(post_reply_resp.is_err());

	let error = post_reply_resp.err().unwrap();

	assert_eq!(
		error.current_context().get_str("code"),
		PostError::MaximumReplyError.get_str("code")
	);
}

async fn setup() -> SetupResponse {
	let db_connection = Arc::new(get_db_connection().await.unwrap());
	let sdk_config = Arc::new(get_aws_sdk_config().await);
	let factory = Factory::new(&db_connection, &sdk_config);
	let post_service = Arc::new(factory.new_post_service());
	let auth_service = Arc::new(factory.new_auth_service());
	let user_service = Arc::new(factory.new_user_service(auth_service));
	Migrator::refresh(&db_connection).await.unwrap();
	let create_post_input = CreatePostInput {
		title: mock_data::post::POST_TITLE.to_owned(),
		category_id: UNCLASSIFIED_CATEGORY_ID.to_owned(),
		content: mock_data::post::POST_CONTENT.to_owned(),
		user_id: None,
	};
	let post = post_service.create(&create_post_input).await.unwrap();
	let user = user_service.get_profile(FAKE_ACCESS_TOKEN).await.unwrap();
	SetupResponse {
		user,
		post,
		post_service,
	}
}

pub struct SetupResponse {
	post: Post,
	user: User,
	post_service: Arc<PostService>,
}
