use strum_macros::{Display, EnumProperty, EnumString};

#[derive(Display, EnumString, EnumProperty)]
pub enum CachingKey {
	#[strum(serialize = "config-categories")]
	CategoryConfigs,
}
