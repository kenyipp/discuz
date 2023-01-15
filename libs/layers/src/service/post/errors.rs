use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum PostError {
	#[display(fmt = "Post Error: Generic {}", _0)]
	Generic(#[error(not(source))] String),
}
