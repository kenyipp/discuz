use strum_macros::EnumString;

#[derive(EnumString, Debug)]
pub enum FileType {
	#[strum(serialize = "avatar_uri")]
	AvatarUri,
	#[strum(serialize = "attachment")]
	Attachment,
}
