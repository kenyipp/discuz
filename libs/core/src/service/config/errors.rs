use derive_more::{Display, Error};
use strum_macros::EnumProperty;

#[derive(Debug, Display, EnumProperty, Error)]
pub enum ConfigError {
	#[display(fmt = "{_0}")]
	#[strum(props(code = "generic"))]
	Generic(#[error(not(source))] String),
	#[display(fmt = "Internal Server Error")]
	#[strum(props(code = "internal_server_error"))]
	InternalServerError,
}
