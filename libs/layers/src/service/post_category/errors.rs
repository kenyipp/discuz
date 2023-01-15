use derive_more::{Display, Error};
use strum_macros::EnumProperty;

#[derive(Debug, Display, EnumProperty, Error)]
pub enum PostCategoryError {
	#[display(fmt = "Post Category Error: Generic {}", _0)]
	#[strum(props(code = "generic"))]
	Generic(#[error(not(source))] String),
	#[display(fmt = "Post Category Error: Target category not exist")]
	#[strum(props(code = "category_not_exist"))]
	CategoryNotExistError,
	#[display(
		fmt = "Post Category Error: Can't create or update the category because the target category name exists"
	)]
	#[strum(props(code = "duplicate_category"))]
	DuplicateCategoryError {
		name: String,
		detail: Option<String>,
	},
	#[strum(props(code = "internal_server_error"))]
	#[display(fmt = "Post Category Error: Internal Server Error")]
	InternalServerError,
}
