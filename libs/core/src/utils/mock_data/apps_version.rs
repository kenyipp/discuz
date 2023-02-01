use crate::repository::database::entities::apps_version;
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;

pub const PACKAGE_ID: &str = "DEFAULT_PACKAGE_ID";

pub async fn create_mock_apps_version(db_connection: &DatabaseConnection) -> String {
	let id = Uuid::new_v4().to_string();

	let apps_version = apps_version::ActiveModel {
		id: Set(id.to_owned()),
		platform: Set("ios".to_owned()),
		package_id: Set(PACKAGE_ID.to_owned()),
		current_version: Set("1.0.0".to_owned()),
		minimal_version: Set("1.0.0".to_owned()),
		..Default::default()
	};
	apps_version::Entity::insert(apps_version)
		.exec(db_connection)
		.await
		.unwrap();

	id
}
