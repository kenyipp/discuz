use aws_sdk_cognitoidentityprovider::model::AttributeType;
use error_stack::{ Report, Result, ResultExt };
use std::slice::Iter;
use crate::service::auth::errors::AuthError;
use crate::service::auth::provider::api_provider::ApiCognitoTrait;
use super::auth_user::AuthUser;

pub async fn get_auth_user_by_access_token(
	api_provider: &dyn ApiCognitoTrait,
	access_token: &str
) -> Result<AuthUser, AuthError> {
	let user = api_provider
		.get_user_by_access_token(access_token).await
		.change_context(
			AuthError::Generic("Unable to retrieve user context by the access token".to_owned())
		)?;
	let attributes = match user.user_attributes() {
		Some(attributes) => attributes.iter(),
		None => {
			return Err(Report::new(AuthError::Generic("Unable to extract attributes".to_owned())));
		}
	};
	let response = AuthUser {
		username: user.username().unwrap().to_owned(),
		sub: get_value_by_key(&attributes, "sub").unwrap(),
		email_verified: get_value_by_key(&attributes, "email_verified").map(
			|email_verified| email_verified == "true"
		),
		name: get_value_by_key(&attributes, "name").unwrap(),
		phone_number_verified: get_value_by_key(&attributes, "phone_number_verified").map(
			|phone_number| phone_number == "true"
		),
		email: get_value_by_key(&attributes, "email").unwrap(),
		picture: get_value_by_key(&attributes, "picture"),
	};
	Ok(response)
}

fn get_value_by_key(attributes: &Iter<AttributeType>, key: &str) -> Option<String> {
	if
		let Some(attribute) = attributes.to_owned().find(|x| {
			if let Some(name) = x.name() { name == key } else { false }
		})
	{
		if let Some(value) = attribute.value() {
			return Some(value.to_owned());
		}
	}
	None
}
