use strum_macros::{Display, EnumString};

#[derive(Display, EnumString, Debug, PartialEq, Eq)]
pub enum UserRole {
	#[strum(serialize = "admin")]
	Admin,
	#[strum(serialize = "user")]
	User,
}
