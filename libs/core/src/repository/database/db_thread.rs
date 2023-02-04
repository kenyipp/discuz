// use crate::constants::UNCLASSIFIED_CATEGORY_ID;
// use sea_orm::{sea_query::Expr, DatabaseConnection, *};
// use std::sync::Arc;

// #[non_exhaustive]
// #[derive(Debug, Clone)]
// pub struct DbThread {
// 	db_connection: Arc<DatabaseConnection>,
// }

// impl DbThread {
// 	pub fn new(db_connection: &Arc<DatabaseConnection>) -> DbThread {
// 		DbThread {
// 			db_connection: db_connection.clone(),
// 		}
// 	}
// }

// #[async_trait]
// pub trait DbPostTrait {
// 	// Post
// 	async fn get_threads(&self, input: &GetThreadsInput);
// }

// pub struct GetThreadsInput {
// 	category_id: String,
// 	last_id: Option<i32>,
// 	limit: i32,
// 	order: String,
// }

// impl Default for GetThreadsInput {
// 	fn default() -> Self {
// 		Self {
// 			category_id: UNCLASSIFIED_CATEGORY_ID.to_owned(),
// 			last_id: None,
// 			limit: 60,
// 			order: "DESC".to_owned(),
// 		}
// 	}
// }
