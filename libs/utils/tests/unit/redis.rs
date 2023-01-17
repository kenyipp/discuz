use discuz_utils::config::Redis;

#[test]
fn test_default_redis_url() {
	let redis = Redis {
		..Default::default()
	};
	assert_eq!(redis.get_connection_string(), "redis://localhost:6379/0");
}

#[test]
fn test_redis_url() {
	let redis = Redis {
		password: Some("123456".to_owned()),
		..Default::default()
	};
	assert_eq!(
		redis.get_connection_string(),
		"redis://:123456@localhost:6379/0"
	);
}
