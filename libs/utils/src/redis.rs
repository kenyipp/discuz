use crate::config::get_config;
use fred::prelude::*;
use tokio::sync::Mutex;

lazy_static! {
	static ref REDIS: Mutex<Option<RedisClient>> = Mutex::new(None);
}

pub async fn get_redis_client() -> Result<Option<RedisClient>, RedisError> {
	let config = get_config();
	if !config.redis.enable {
		return Ok(None);
	}
	let mut redis = REDIS.lock().await;
	if redis.is_none() {
		let client = get_redis_connection().await?;
		*redis = client;
	}
	Ok(redis.clone())
}

pub async fn get_redis_connection() -> Result<Option<RedisClient>, RedisError> {
	let config = get_config();

	if !config.redis.enable {
		return Ok(None);
	}

	let connection_string = config.redis.get_connection_string();

	let config = RedisConfig::from_url(&connection_string)?;
	let policy = ReconnectPolicy::default();
	let client = RedisClient::new(config);

	let _ = client.connect(Some(policy));
	client.wait_for_connect().await?;

	Ok(Some(client))
}
