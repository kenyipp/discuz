use std::sync::Arc;
use strum::EnumProperty;

use discuz_core::{
	constants::{FAKE_ACCESS_TOKEN, UNCLASSIFIED_CATEGORY_ID},
	migration::{Migrator, MigratorTrait},
	repository::{
		database::{db_post_reply::DbPostReply, db_user::DbUser},
		repo_post_reply::RepoPostReply,
		repo_user::RepoUser,
	},
	service::{
		factory::Factory,
		post::post_service::{
			CreatePostInput, Post, PostService, PostServiceTrait, UpdatePostInput,
		},
		post_reply::{
			errors::PostReplyError,
			post_reply_service::{CreateCommentInput, PostReplyService, PostReplyServiceTrait},
		},
		user::user_service::{User, UserService, UserServiceTrait},
	},
	utils::mock_data,
};
use discuz_utils::{amazon::get_aws_sdk_config, get_db_connection};

#[tokio::test]
async fn create_post_reply() {
	let SetupResponse {
		post,
		user,
		post_service,
		post_reply_service,
		..
	} = setup().await;

	let input = CreateCommentInput {
		post_id: post.id.to_owned(),
		content: mock_data::post::COMMENT_CONTENT.to_owned(),
		quote_reply_id: None,
		user_id: user.id.to_owned(),
	};

	let post_reply = post_reply_service.create(&input).await;
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
		post_reply_service,
		..
	} = setup().await;

	let input = CreateCommentInput {
		post_id: post.id.to_owned(),
		content: mock_data::post::COMMENT_CONTENT.to_owned(),
		quote_reply_id: None,
		user_id: user.id.to_owned(),
	};

	let post_reply = post_reply_service.create(&input).await.unwrap();
	post_reply_service.delete(post_reply.id).await.unwrap();

	let post_reply = post_reply_service
		.find_by_id(post_reply.id)
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
		post_reply_service,
		..
	} = setup().await;

	let input = UpdatePostInput {
		id: post.id,
		max_comment_count: Some(1),
		title: mock_data::post::POST_TITLE.to_owned(),
		post_category_id: UNCLASSIFIED_CATEGORY_ID.to_owned(),
		content: mock_data::post::POST_CONTENT.to_owned(),
		status_id: None,
		user_id: None,
	};

	post_service.update(&input).await.unwrap();

	let input = CreateCommentInput {
		post_id: post.id.to_owned(),
		content: mock_data::post::COMMENT_CONTENT.to_owned(),
		quote_reply_id: None,
		user_id: user.id.to_owned(),
	};

	let post_reply_resp = post_reply_service.create(&input).await;
	assert!(post_reply_resp.is_ok());

	let post_reply_resp = post_reply_service.create(&input).await;
	assert!(post_reply_resp.is_err());

	let error = post_reply_resp.err().unwrap();

	assert_eq!(
		error.current_context().get_str("code"),
		PostReplyError::MaximumCommentError.get_str("code")
	);
}

async fn setup() -> SetupResponse {
	let db_connection = Arc::new(get_db_connection().await.unwrap());
	let sdk_config = Arc::new(get_aws_sdk_config().await);
	let factory = Factory::new(&db_connection, &sdk_config);
	let db_post_reply = DbPostReply::new(&db_connection);
	let repo_post_reply = RepoPostReply::new(db_post_reply);
	let post_service = Arc::new(factory.new_post_service());
	let post_reply_service = Arc::new(PostReplyService {
		repo_post_reply,
		post_service: post_service.clone(),
	});
	let db_user = DbUser::new(&db_connection);
	let repo_user = RepoUser::new(db_user);
	let factory = Factory::new(&db_connection, &sdk_config);
	let auth_service = Arc::new(factory.new_auth_service());
	let user_service = Arc::new(UserService {
		repo_user,
		auth_service: auth_service.clone(),
	});
	Migrator::refresh(&db_connection).await.unwrap();
	let create_post_input = CreatePostInput {
		title: mock_data::post::POST_TITLE.to_owned(),
		post_category_id: UNCLASSIFIED_CATEGORY_ID.to_owned(),
		content: mock_data::post::POST_CONTENT.to_owned(),
		user_id: None,
	};
	let post = post_service.create(&create_post_input).await.unwrap();
	let user = user_service.get_profile(FAKE_ACCESS_TOKEN).await.unwrap();
	SetupResponse {
		user,
		post,
		post_service,
		post_reply_service,
	}
}

pub struct SetupResponse {
	post: Post,
	user: User,
	post_service: Arc<PostService>,
	post_reply_service: Arc<PostReplyService>,
}
