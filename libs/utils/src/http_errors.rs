use derive_more::Display;
use error_stack::{IntoReport, Result, ResultExt};
use serde::Deserialize;
use serde_json;
use std::{env::current_dir, error::Error, fs::File, io::Read};
use tokio::sync::Mutex;

lazy_static! {
	static ref HTTP_ERRORS: Mutex<Option<Vec<HttpError>>> = Mutex::new(None);
}

pub async fn get_http_error(code: u32) -> Result<Option<HttpError>, GetHttpError> {
	let mut http_errors = get_http_errors().await?.into_iter();
	Ok(http_errors.find(|x| x.code == code))
}

pub async fn get_http_errors() -> Result<Vec<HttpError>, GetHttpError> {
	if let Some(cache) = HTTP_ERRORS.lock().await.clone() {
		return Ok(cache);
	}

	let mut data = String::new();

	let mut path = current_dir()
		.into_report()
		.change_context(GetHttpError::IOError("Unable to get the path".to_string()))?;

	path.push("dict/http-errors.json");

	File::open(path)
		.into_report()
		.change_context(GetHttpError::IOError("Unable to open the file".to_string()))?
		.read_to_string(&mut data)
		.into_report()
		.change_context(GetHttpError::IOError(
			"Unable to read the string from file".to_string(),
		))?;

	let errors: Vec<HttpError> = serde_json::from_str(&data)
		.into_report()
		.change_context(GetHttpError::ParseError)?;

	Ok(errors)
}

#[derive(Debug, Display)]
pub enum GetHttpError {
	#[display(fmt = "Generic Error")]
	GenericError,
	#[display(fmt = "{}", _0)]
	IOError(String),
	#[display(fmt = "Malformed JSON")]
	ParseError,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HttpError {
	pub name: String,
	pub code: u32,
	pub message: String,
	#[serde(rename(deserialize = "type"))]
	pub error_type: String,
}

impl Error for GetHttpError {}
