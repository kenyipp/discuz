use strum_macros::{Display, EnumMessage, EnumString};

#[derive(Display, Debug, EnumString, EnumMessage)]
pub enum UserStatus {
	#[strum(serialize = "normal")]
	Normal,
	#[strum(
		serialize = "banned",
		detailed_message = "Banned users are unable to create or reply to posts"
	)]
	Banned,
	#[strum(
		serialize = "deactivated",
		detailed_message = "Deactivated users are no longer able to log in to the forum. Their previous posts and comments will be archived for reference, but will not be visible on the active site. "
	)]
	Deactivated,
}
