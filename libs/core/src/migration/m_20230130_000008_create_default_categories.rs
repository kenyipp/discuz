use crate::{
	constants::UNCLASSIFIED_CATEGORY_ID, repository::database::entities::def_post_category,
};
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
	child: Vec<Category>,
}

#[async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		let categories = vec![
			Category {
				name: "Life".to_owned(),
				child: vec![
					Category {
						name: "Creative".to_owned(),
						child: vec![
							Category {
								name: "Unclassified".to_owned(),
								child: vec![],
							},
							Category {
								name: "Remix".to_owned(),
								child: vec![],
							},
							Category {
								name: "Art Design".to_owned(),
								child: vec![],
							},
						],
					},
					Category {
						name: "Health".to_owned(),
						child: vec![],
					},
					Category {
						name: "Diet".to_owned(),
						child: vec![],
					},
					Category {
						name: "Love".to_owned(),
						child: vec![],
					},
				],
			},
			Category {
				name: "Hobby".to_owned(),
				child: vec![
					Category {
						name: "Sport".to_owned(),
						child: vec![],
					},
					Category {
						name: "Academic".to_owned(),
						child: vec![Category {
							name: "Mysterious".to_owned(),
							child: vec![],
						}],
					},
					Category {
						name: "Story".to_owned(),
						child: vec![],
					},
					Category {
						name: "Game".to_owned(),
						child: vec![],
					},
				],
			},
			Category {
				name: "Other".to_owned(),
				child: vec![
					Category {
						name: "Admin".to_owned(),
						child: vec![],
					},
					Category {
						name: "Black Hole".to_owned(),
						child: vec![],
					},
				],
			},
		];
		create_categories(manager, categories, None, None).await;
		Ok(())
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		def_post_category::Entity::update_many()
			.col_expr(def_post_category::Column::StatusId, Expr::value("D"))
			.filter(def_post_category::Column::Id.ne(UNCLASSIFIED_CATEGORY_ID))
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
	let id = Uuid::new_v4().to_string();
	let category = def_post_category::ActiveModel {
		id: Set(id.to_owned()),
		name: Set(category.name.to_owned()),
		slug: Set(slugify!(&category.name)),
		parent_id: Set(parent_id.to_owned()),
		postable: Set(level == 3),
		level: Set(level.to_owned()),
		..def_post_category::ActiveModel::default()
	};
	def_post_category::Entity::insert(category)
		.exec(manager.get_connection())
		.await
		.unwrap();
	id
}
