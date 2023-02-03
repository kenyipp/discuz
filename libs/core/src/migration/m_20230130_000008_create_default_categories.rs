use crate::{constants::UNCLASSIFIED_CATEGORY_ID, repository::database::entities::category};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use sea_orm_migration::prelude::*;
use slugify::slugify;
use std::vec;
use uuid::Uuid;

pub struct Migration;

impl MigrationName for Migration {
	fn name(&self) -> &str {
		"m_20230130_000008_create_default_categories"
	}
}

#[derive(Debug, Clone)]
struct Category {
	name: String,
	sort_index: i32,
	child: Vec<Category>,
}

#[async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		let categories = vec![
			Category {
				name: "Chit chat".to_owned(),
				sort_index: 10,
				child: vec![],
			},
			Category {
				name: "Daily Life".to_owned(),
				sort_index: 10,
				child: vec![
					Category {
						name: "Creative".to_owned(),
						sort_index: 11,
						child: vec![
							Category {
								name: "Unclassified".to_owned(),
								sort_index: 12,
								child: vec![],
							},
							Category {
								name: "Remix".to_owned(),
								sort_index: 13,
								child: vec![],
							},
							Category {
								name: "Art and literature".to_owned(),
								sort_index: 14,
								child: vec![],
							},
						],
					},
					Category {
						name: "Health and wellness".to_owned(),
						sort_index: 15,
						child: vec![],
					},
					Category {
						name: "Food and cooking".to_owned(),
						sort_index: 16,
						child: vec![],
					},
					Category {
						name: "Relationships and dating".to_owned(),
						sort_index: 17,
						child: vec![],
					},
				],
			},
			Category {
				name: "Hobbies and interests".to_owned(),
				sort_index: 20,
				child: vec![
					Category {
						name: "Sports and recreation".to_owned(),
						sort_index: 21,
						child: vec![],
					},
					Category {
						name: "Academic".to_owned(),
						sort_index: 22,
						child: vec![Category {
							name: "Mysterious".to_owned(),
							sort_index: 23,
							child: vec![],
						}],
					},
					Category {
						name: "Story".to_owned(),
						sort_index: 24,
						child: vec![],
					},
					Category {
						name: "Gaming".to_owned(),
						sort_index: 25,
						child: vec![],
					},
				],
			},
			Category {
				name: "Others".to_owned(),
				sort_index: 30,
				child: vec![
					Category {
						name: "Admin".to_owned(),
						sort_index: 31,
						child: vec![],
					},
					Category {
						name: "Black Hole".to_owned(),
						sort_index: 32,
						child: vec![],
					},
				],
			},
		];
		create_categories(manager, categories, None, None).await;
		Ok(())
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		category::Entity::update_many()
			.col_expr(category::Column::StatusId, Expr::value("D"))
			.filter(category::Column::Id.ne(UNCLASSIFIED_CATEGORY_ID))
			.exec(manager.get_connection())
			.await
			.unwrap();
		Ok(())
	}
}

#[async_recursion]
async fn create_categories(
	manager: &SchemaManager<'_>,
	categories: Vec<Category>,
	parent_id: Option<String>,
	level: Option<i32>,
) {
	let level = level.unwrap_or(1);
	for category in categories.iter() {
		let category_id = insert_category(manager, category, parent_id.to_owned(), level).await;
		create_categories(
			manager,
			category.child.clone(),
			Some(category_id.to_owned()),
			Some(level + 1),
		)
		.await;
	}
}

async fn insert_category(
	manager: &SchemaManager<'_>,
	category: &Category,
	parent_id: Option<String>,
	level: i32,
) -> String {
	let id = if category.name == "Chit chat" {
		UNCLASSIFIED_CATEGORY_ID.to_owned()
	} else {
		Uuid::new_v4().to_string()
	};
	let category = category::ActiveModel {
		id: Set(id.to_owned()),
		name: Set(category.name.to_owned()),
		slug: Set(slugify!(&category.name)),
		parent_id: Set(parent_id.to_owned()),
		sort_index: Set(category.sort_index),
		postable: Set(category.child.is_empty()),
		level: Set(level.to_owned()),
		..category::ActiveModel::default()
	};
	category::Entity::insert(category)
		.exec(manager.get_connection())
		.await
		.unwrap();
	id
}
