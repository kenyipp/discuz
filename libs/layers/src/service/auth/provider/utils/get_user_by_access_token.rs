use aws_sdk_cognitoidentityprovider::{ Client, output::GetUserOutput };
use error_stack::{ IntoReport, Result, ResultExt };
use super::super::errors::ProviderError;

pub async fn get_user_by_access_token(
	client: &Client,
	access_token: &str
) -> Result<GetUserOutput, ProviderError> {
	let result = client
		.get_user()
		.access_token(access_token)
		.send().await
		.into_report()
		.change_context(ProviderError::InvalidAccessToken)
		.attach_printable(
			"Unable to retrieve the user context from the Cognito by the access token"
		);
	result
}
