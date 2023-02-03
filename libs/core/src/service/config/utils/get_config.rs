use error_stack::{Result, ResultExt};
use std::string::ToString;
use tokio::try_join;

use crate::{
	repository::{
		database::category::Category,
		repo_config::{AppsVersion, RepoConfig, RepoConfigTrait},
	},
	service::config::{constants::AppStatus, errors::ConfigError},
};

pub async fn execute(repo_config: &RepoConfig) -> Result<AppConfig, ConfigError> {
	let (versions, categories, configs) = try_join!(
		repo_config.get_apps_versions(),
		repo_config.get_categories(),
		repo_config.get_configs()
	)
	.change_context(ConfigError::InternalServerError)?;

	let categories = parse_categories_into_tree(categories);

	let app_status = configs
		.iter()
		.find(|config| config.key == "app_status")
		.map(|config| config.value.to_owned())
		.unwrap_or_else(|| AppStatus::Maintaining.to_string());

	let app_maintaining_message = configs
		.iter()
		.find(|config| config.key == "app_maintaining_message")
		.map(|config| config.value.to_owned());

	let app_config = AppConfig {
		app_status,
		app_maintaining_message,
		versions,
		categories,
	};

	Ok(app_config)
}

fn parse_categories_into_tree(categories: Vec<Category>) -> Vec<DtoCategory> {
	let mut root_categories: Vec<DtoCategory> = vec![];
	let mut middle_categories: Vec<DtoCategory> = vec![];

	for category in categories.iter() {
		let dto_category = DtoCategory::from(category.to_owned());
		match category.level {
			1 => {
				root_categories.push(dto_category);
			}
			2 => {
				middle_categories.push(dto_category);
			}
			3 => {
				if let Some(parent_id) = category.parent_id.to_owned() {
					if let Some(index) = middle_categories
						.iter()
						.position(|category| category.id == parent_id)
					{
						let parent = middle_categories.get_mut(index).unwrap();
						parent.sub_categories.push(dto_category);
					}
				}
			}
			level => panic!("The category level provided, level {level}, is not valid"),
		};
	}

	for raw_category in categories.iter().filter(|category| category.level == 2) {
		if let Some(parent_id) = raw_category.parent_id.to_owned() {
			let category = middle_categories
				.iter()
				.find(|category| category.id == raw_category.id)
				.unwrap()
				.clone();
			let parent_index = root_categories
				.iter()
				.position(|category| category.id == parent_id)
				.unwrap();
			let parent = root_categories.get_mut(parent_index).unwrap();
			parent.sub_categories.push(category);
		}
	}

	root_categories
}

pub struct AppConfig {
	pub app_status: String,
	pub app_maintaining_message: Option<String>,
	pub versions: Vec<AppsVersion>,
	pub categories: Vec<DtoCategory>,
}

pub struct Configs {
	pub app_status: AppStatus,
	pub app_maintaining_message: String,
}

#[derive(Clone)]
pub struct DtoCategory {
	pub id: String,
	pub name: String,
	pub slug: String,
	pub description: Option<String>,
	pub postable: bool,
	pub sub_categories: Vec<DtoCategory>,
}

impl From<Category> for DtoCategory {
	fn from(category: Category) -> Self {
		DtoCategory {
			id: category.id,
			name: category.name.to_owned(),
			slug: category.slug.to_owned(),
			description: category.description.to_owned(),
			postable: category.postable.to_owned(),
			sub_categories: Vec::new(),
		}
	}
}
