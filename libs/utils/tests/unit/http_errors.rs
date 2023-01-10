use discuz_utils::http_errors::{get_http_error, get_http_errors};

#[tokio::test]
async fn test_get_http_errors() {
	let errors = get_http_errors().await.unwrap();
	assert!(!errors.is_empty());
}

#[tokio::test]
async fn test_get_http_error() {
	let exist_error = get_http_error(400).await.unwrap();
	assert!(exist_error.is_some());
	let exist_error = exist_error.unwrap();
	assert_eq!(exist_error.code, 400);
	assert_eq!(exist_error.error_type, "BadRequest");

	let non_exist_error = get_http_error(200).await.unwrap();
	assert!(non_exist_error.is_none());
}
