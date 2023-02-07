use actix_web::http::header::HeaderValue;
use inflector::cases::kebabcase::to_kebab_case;
use std::{fmt, time::Duration};

pub trait PolicyTrait {
	fn get_header_value(&self) -> HeaderValue;
}

#[derive(Clone)]
pub struct XXssProtection {
	pub enable: bool,
	pub block_mode: Option<bool>,
	pub report_uri: Option<String>,
}

impl Default for XXssProtection {
	fn default() -> Self {
		Self {
			enable: false,
			block_mode: Some(false),
			report_uri: None,
		}
	}
}

impl PolicyTrait for XXssProtection {
	fn get_header_value(&self) -> HeaderValue {
		let string_value = if self.enable {
			let config = self.clone();
			let mut values: Vec<String> = vec!["1".to_owned()];
			if let Some(block_mode) = config.block_mode {
				if block_mode {
					values.push("mode=block".to_owned());
				}
			}
			if let Some(report_uri) = config.report_uri {
				values.push(format!("report={report_uri}"));
			}
			values.join(";")
		} else {
			"0".to_owned()
		};
		HeaderValue::from_str(&string_value).unwrap()
	}
}

#[derive(Debug)]
pub enum ReferrerPolicy {
	/// Instructs the browser to not send the Referer header.
	NoReferrer,
	/// Instructs the browser to send the full URL in the Referer header only if the request is made to a secure origin (HTTPS), and to not send the Referer header if the request is made to a non-secure origin (HTTP)
	NoReferrerWhenDowngrade,
	/// Instructs the browser to only send the Referer header if the request is made to the same origin as the source page.
	SameOrigin,
	/// Instructs the browser to only send the origin (scheme, host, and port) of the source page in the Referer header.
	Origin,
	/// Instructs the browser to only send the origin (scheme, host, and port) of the source page in the Referer header, but only if the request is made to a secure origin (HTTPS).
	StrictOrigin,
	/// Instructs the browser to only send the origin (scheme, host, and port) of the source page in the Referer header if the request is cross-origin.
	OriginWhenCrossOrigin,
	/// Instructs the browser to only send the origin (scheme, host, and port) of the source page in the Referer header if the request is cross-origin and made to a secure origin (HTTPS).
	StrictOriginWhenCrossOrigin,
	/// Instructs the browser to send the full URL in the Referer header, regardless of the origin of the request or the source page.
	UnsafeUrl,
}

impl fmt::Display for ReferrerPolicy {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", to_kebab_case(&format!("{:?}", self)))
	}
}

impl Default for ReferrerPolicy {
	fn default() -> Self {
		Self::NoReferrer
	}
}

impl PolicyTrait for ReferrerPolicy {
	fn get_header_value(&self) -> HeaderValue {
		HeaderValue::from_str(&self.to_string()).unwrap()
	}
}

pub enum XFrameOptions {
	SameOrigin,
	Deny,
	AllowFrom(String),
}

impl Default for XFrameOptions {
	fn default() -> Self {
		Self::SameOrigin
	}
}

impl PolicyTrait for XFrameOptions {
	fn get_header_value(&self) -> HeaderValue {
		let option_value = match self {
			XFrameOptions::SameOrigin => "SAMEORIGIN".to_owned(),
			XFrameOptions::Deny => "DENY".to_owned(),
			XFrameOptions::AllowFrom(origin) => format!("ALLOW-FROM: {origin}"),
		};
		HeaderValue::from_str(&option_value).unwrap()
	}
}

#[derive(Debug)]
pub enum XPermittedCrossDomainPolicies {
	/// No cross-domain requests are allowed
	None,
	/// Only Adobe Flash Player is allowed to make cross-domain requests
	MasterOnly,
	/// Cross-domain requests are allowed based on the MIME type of the requested resource
	ByContentType,
	/// All cross-domain requests are allowed
	All,
	/// Cross-domain requests are allowed based on the file type of the requested resource
	ByFtpFile,
}

impl fmt::Display for XPermittedCrossDomainPolicies {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", to_kebab_case(&format!("{:?}", self)))
	}
}

impl Default for XPermittedCrossDomainPolicies {
	fn default() -> Self {
		Self::None
	}
}

impl PolicyTrait for XPermittedCrossDomainPolicies {
	fn get_header_value(&self) -> HeaderValue {
		HeaderValue::from_str(&self.to_string()).unwrap()
	}
}

#[derive(Debug)]
pub enum XDnsPrefetchControl {
	On,
	Off,
}

impl Default for XDnsPrefetchControl {
	fn default() -> Self {
		Self::Off
	}
}

impl fmt::Display for XDnsPrefetchControl {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", &format!("{:?}", self).to_lowercase())
	}
}

impl PolicyTrait for XDnsPrefetchControl {
	fn get_header_value(&self) -> HeaderValue {
		HeaderValue::from_str(&self.to_string()).unwrap()
	}
}

#[derive(PartialEq)]
pub enum XDownloadOptions {
	Enable,
	Disable,
}

#[derive(PartialEq)]
pub enum XContentTypeOptions {
	Enable,
	Disable,
}

#[derive(Clone)]
pub struct StrictTransportSecurity {
	pub(crate) max_age: u32,
	pub(crate) include_sub_domains: bool,
	pub(crate) preload: bool,
}

impl Default for StrictTransportSecurity {
	fn default() -> Self {
		Self {
			max_age: 31536000,
			include_sub_domains: true,
			preload: false,
		}
	}
}

impl PolicyTrait for StrictTransportSecurity {
	fn get_header_value(&self) -> HeaderValue {
		let mut values: Vec<String> = vec![format!("max-age={}", self.max_age)];
		if self.include_sub_domains {
			values.push("includeSubDomains".to_owned());
		}
		if self.preload {
			values.push("preload".to_owned());
		}
		HeaderValue::from_str(&values.join("; ")).unwrap()
	}
}
