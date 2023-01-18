use aws_sdk_cognitoidentityprovider::{output::GetUserOutput, Client};
use error_stack::{IntoReport, Result, ResultExt};

use crate::service::auth::provider::errors::ProviderError;

pub async fn get_user_by_access_token(
	client: &Client,
	access_token: &str,
) -> Result<GetUserOutput, ProviderError> {
	client
		.get_user()
		.access_token(access_token)
		.send()
		.await
		.into_report()
		.change_context(ProviderError::InvalidAccessToken)
		.attach_printable(
			"Unable to retrieve the user context from the Cognito by the access token",
		)
}
