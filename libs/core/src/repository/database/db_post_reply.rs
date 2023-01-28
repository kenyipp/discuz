use chrono;
use sea_orm::{sea_query::SimpleExpr, DatabaseConnection, *};
use std::sync::Arc;

pub use super::entities::post_reply::PostReply;
use super::entities::{post, post_reply};

#[derive(Debug, Clone)]
pub struct DbPostReply {
	db_connection: Arc<DatabaseConnection>,
}

impl DbPostReply {
	pub fn new(db_connection: &Arc<DatabaseConnection>) -> DbPostReply {
		DbPostReply {
			db_connection: db_connection.clone(),
		}
	}
}

#[async_trait]
pub trait DbPostReplyTrait {
	async fn find_by_id(&self, id: i32) -> Result<Option<PostReply>, DbErr>;
	async fn create(&self, input: &CreateCommentInput) -> Result<i32, DbErr>;
	async fn delete(&self, id: i32) -> Result<(), DbErr>;
}

#[async_trait]
impl DbPostReplyTrait for DbPostReply {
	async fn find_by_id(&self, id: i32) -> Result<Option<PostReply>, DbErr> {
		post_reply::Entity::find()
			.filter(post_reply::Column::Id.eq(id))
			.one(&*self.db_connection)
			.await
	}

	async fn create(&self, input: &CreateCommentInput) -> Result<i32, DbErr> {
		let post_reply = post_reply::ActiveModel {
			post_id: Set(input.post_id.to_owned()),
			content: Set(input.content.to_owned()),
			quote_reply_id: Set(input.quote_reply_id.to_owned()),
			user_id: Set(input.user_id.to_owned()),
			..Default::default()
		};

		// Increase the post's comment count by 1
		Update::many(post::Entity)
			.col_expr(
				post::Column::CommentCount,
				SimpleExpr::Custom(format!("{} + 1", post::Column::CommentCount.to_string())),
			)
			.filter(post::Column::Id.eq(input.post_id.to_owned()))
			.exec(&*self.db_connection)
			.await?;

		let result = post_reply::Entity::insert(post_reply)
			.exec(&*self.db_connection)
			.await?;
		Ok(result.last_insert_id)
	}

	async fn delete(&self, id: i32) -> Result<(), DbErr> {
		let mut post_reply: post_reply::ActiveModel = self
			.find_by_id(id)
			.await?
			.ok_or_else(|| DbErr::Custom(format!("Invalid post comment #{id}")))?
			.into();

		if post_reply.status_id.take() == Some("D".to_owned()) {
			return Err(DbErr::Custom(
				"The post comment has been deleted before".to_owned(),
			));
		}
		post_reply.status_id = Set("D".to_owned());
		post_reply.updated_at = Set(chrono::offset::Utc::now());
		post_reply.update(&*self.db_connection).await?;

		Ok(())
	}
}

pub struct CreateCommentInput {
	pub post_id: i32,
	pub quote_reply_id: Option<i32>,
	pub content: String,
	pub user_id: String,
}
