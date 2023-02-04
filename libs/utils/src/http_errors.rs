use serde::Deserialize;
use serde_json;
use std::{
	env::current_dir,
	fs::File,
	io::{Error as IOError, Read},
};
use tokio::sync::Mutex;

lazy_static! {
	static ref HTTP_ERRORS: Mutex<Option<Vec<HttpError>>> = Mutex::new(None);
}

pub async fn get_http_error(code: u32) -> Result<Option<HttpError>, IOError> {
	let mut http_errors = get_http_errors().await?.into_iter();
	Ok(http_errors.find(|x| x.code == code))
}

pub async fn get_http_errors() -> Result<Vec<HttpError>, IOError> {
	if let Some(cache) = HTTP_ERRORS.lock().await.clone() {
		return Ok(cache);
	}

	let mut data = String::new();
	let mut path = current_dir()?;
	path.push("dict/http-errors.json");
	File::open(path)?.read_to_string(&mut data)?;

	let errors: Vec<HttpError> = serde_json::from_str(&data)?;

	// Save the http errors into mutex block
	let mut lock = HTTP_ERRORS.lock().await;
	*lock = Some(errors.clone());

	Ok(errors)
}

#[derive(Clone, Deserialize)]
pub struct HttpError {
	pub name: String,
	pub code: u32,
	pub message: String,
	#[serde(rename(deserialize = "type"))]
	pub error_type: String,
}
