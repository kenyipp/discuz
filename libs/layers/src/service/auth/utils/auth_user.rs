#[derive(Debug, Clone)]
pub struct AuthUser {
	pub sub: String,
	pub username: String,
	pub name: String,
	pub email: String,
	pub picture: Option<String>,
	pub email_verified: Option<bool>,
	pub phone_number_verified: Option<bool>,
}
