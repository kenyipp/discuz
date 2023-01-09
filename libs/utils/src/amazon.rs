use dotenv::dotenv;
use aws_config::{ self, SdkConfig };

pub async fn get_aws_sdk_config() -> SdkConfig {
	dotenv().ok();
	aws_config::load_from_env().await
}
