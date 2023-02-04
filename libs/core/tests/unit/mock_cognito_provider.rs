use discuz_core::{
	service::auth::provider::api_provider::ApiCognitoTrait,
	utils::mock_data::mock_cognito_provider::MockApiCognito,
};
use uuid::Uuid;

#[tokio::test]
async fn should_return_same_user() {
	let user_token = Uuid::new_v4().to_string();
	let api_provider = MockApiCognito;

	let response_a = api_provider
		.get_user_by_access_token(&user_token)
		.await
		.unwrap();

	let response_b = api_provider
		.get_user_by_access_token(&user_token)
		.await
		.unwrap();

	let response_a_name = response_a
		.user_attributes()
		.unwrap()
		.to_owned()
		.iter()
		.find(|attribute| attribute.name().to_owned().unwrap() == "name")
		.unwrap()
		.value()
		.unwrap()
		.to_owned();

	let response_b_name = response_b
		.user_attributes()
		.unwrap()
		.to_owned()
		.iter()
		.find(|attribute| attribute.name().to_owned().unwrap() == "name")
		.unwrap()
		.value()
		.unwrap()
		.to_owned();

	assert_eq!(response_a_name, response_b_name);
}
