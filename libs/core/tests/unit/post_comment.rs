use std::sync::Arc;

use discuz_core::{
	constants::{FAKE_ACCESS_TOKEN, UNCLASSIFIED_CATEGORY_ID},
	migration::{Migrator, MigratorTrait},
	repository::{
		database::{db_post_comment::DbPostComment, db_user::DbUser},
		repo_post_comment::RepoPostComment,
		repo_user::RepoUser,
	},
	service::{
		factory::Factory,
		post::post_service::{CreatePostInput, Post, PostService, PostServiceTrait},
		post_comment::post_comment_service::{
			CreateCommentInput, PostCommentService, PostCommentServiceTrait,
		},
		user::user_service::{User, UserService, UserServiceTrait},
	},
	utils::mock_data,
};
use discuz_utils::{amazon::get_aws_sdk_config, get_db_connection};

#[tokio::test]
async fn create_post_comment() {
	let SetupResponse {
		post,
		user,
		post_service,
		post_comment_service,
		..
	} = setup().await;

	let input = CreateCommentInput {
		post_id: post.id.to_owned(),
		content: mock_data::post::COMMENT_CONTENT.to_owned(),
		quote_comment_id: None,
		user_id: user.id.to_owned(),
	};

	let post_comment = post_comment_service.create(&input).await;
	assert!(post_comment.is_ok());

	let post_comment = post_comment.unwrap();
	assert_eq!(
		post_comment.content,
		mock_data::post::COMMENT_CONTENT.to_owned()
	);

	let post = post_service.find_by_id(post.id).await.unwrap().unwrap();
	assert_eq!(post.comment_count, 1);
}

#[tokio::test]
async fn delete_post_comment() {
	let SetupResponse {
		post,
		user,
		post_service,
		post_comment_service,
		..
	} = setup().await;

	let input = CreateCommentInput {
		post_id: post.id.to_owned(),
		content: mock_data::post::COMMENT_CONTENT.to_owned(),
		quote_comment_id: None,
		user_id: user.id.to_owned(),
	};

	let post_comment = post_comment_service.create(&input).await.unwrap();
	post_comment_service.delete(post_comment.id).await.unwrap();

	let post_comment = post_comment_service
		.find_by_id(post_comment.id)
		.await
		.unwrap()
		.unwrap();
	assert_eq!(post_comment.status_id, "D");

	let post = post_service.find_by_id(post.id).await.unwrap().unwrap();
	assert_eq!(post.comment_count, 1);
}

async fn setup() -> SetupResponse {
	let db_connection = Arc::new(get_db_connection().await.unwrap());
	let sdk_config = Arc::new(get_aws_sdk_config().await);
	let factory = Factory::new(&db_connection, &sdk_config);
	let db_post_comment = DbPostComment::new(&db_connection);
	let repo_post_comment = RepoPostComment::new(db_post_comment);
	let post_service = Arc::new(factory.new_post_service());
	let post_comment_service = Arc::new(PostCommentService { repo_post_comment });
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
		post_comment_service,
	}
}

pub struct SetupResponse {
	post: Post,
	user: User,
	post_service: Arc<PostService>,
	post_comment_service: Arc<PostCommentService>,
}
