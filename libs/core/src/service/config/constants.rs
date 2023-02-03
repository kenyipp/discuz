use strum_macros::{Display, EnumString};

#[derive(Display, EnumString, Debug, PartialEq, Eq)]
pub enum AppStatus {
	#[strum(serialize = "normal")]
	Normal,
	#[strum(serialize = "maintaining")]
	Maintaining,
}
