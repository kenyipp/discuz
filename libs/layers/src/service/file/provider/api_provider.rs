use std::{fmt::Debug, sync::Arc};

use aws_config::SdkConfig;
use error_stack::Result;

use crate::service::file::errors::FileError;
