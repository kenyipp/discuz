use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum ProviderError {
	#[display(fmt = "Provider Error: Generic {}", _0)]
	Generic(#[error(not(source))] String),
	#[display(fmt = "Provider Error: Invalid Presigning Config")]
	InvalidPresigningConfig,
	#[display(fmt = "Provider Error: Invalid Presigned Url Request")]
	InvalidPresignedUrlRequest,
}
