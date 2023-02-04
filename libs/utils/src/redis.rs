use crate::config::get_config;
use fred::prelude::*;
use serde::{de::DeserializeOwned, Serialize};
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

pub async fn get_cached_result<T>(key: &str) -> Option<T>
where
	T: DeserializeOwned,
{
	if let Some(json_string) = get_stringified_json(key).await {
		let json: Option<T> = serde_json::from_str(&json_string).unwrap_or_else(|error| {
			println!("{:#?}", error);
			None
		});
		return json;
	}
	None
}

pub async fn set_cached_result<T: Serialize>(key: &str, json: T, expire: Option<Expiration>) {
	if let Ok(json_string) = serde_json::to_string(&json) {
		set_stringified_json(key, &json_string, expire).await;
	}
}

pub async fn get_stringified_json(key: &str) -> Option<String> {
	if let Ok(Some(redis)) = get_redis_client().await {
		let result: Option<String> = redis.get(key).await.unwrap_or(None);
		return result;
	}
	None
}

pub async fn set_stringified_json(key: &str, value: &str, expire: Option<Expiration>) {
	if let Ok(Some(redis)) = get_redis_client().await {
		redis
			.set(key, value, expire, None, false)
			.await
			.unwrap_or(());
	}
}
