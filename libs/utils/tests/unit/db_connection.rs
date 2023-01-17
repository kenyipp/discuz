// TODO: More test cases should be added to test the ability to get the connection url

use discuz_utils::{config::Database, get_connection_string};

#[test]
fn test_database_connection_url() {
	let config = Database {
		hostname: "localhost".to_owned(),
		username: "mysql".to_owned(),
		password: "password".to_owned(),
		port: 3306,
		database: "database".to_owned(),
		url: None,
	};
	assert_eq!(
		get_connection_string(&config),
		"mysql://mysql:password@localhost:3306/database"
	);
}
