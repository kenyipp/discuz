use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum FileError {
	#[display(fmt = "File Error: Generic {}", _0)]
	Generic(#[error(not(source))] String),
}
