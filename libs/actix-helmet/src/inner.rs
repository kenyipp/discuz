use crate::policies::{
	ReferrerPolicy, StrictTransportSecurity, XContentTypeOptions, XDnsPrefetchControl,
	XDownloadOptions, XFrameOptions, XPermittedCrossDomainPolicies, XXssProtection,
};
use std::default::Default;

pub struct Inner {
	pub x_xss_protection: XXssProtection,
	pub referrer_policy: ReferrerPolicy,
	pub x_frame_options: XFrameOptions,
	pub x_permitted_cross_domain_policies: XPermittedCrossDomainPolicies,
	pub x_dn_prefetch_control: XDnsPrefetchControl,
	pub x_download_options: XDownloadOptions,
	pub x_content_type_options: XContentTypeOptions,
	pub strict_transport_security: Option<StrictTransportSecurity>,
}

impl Default for Inner {
	fn default() -> Self {
		Inner {
			x_xss_protection: XXssProtection::default(),
			referrer_policy: ReferrerPolicy::default(),
			x_frame_options: XFrameOptions::default(),
			x_permitted_cross_domain_policies: XPermittedCrossDomainPolicies::default(),
			x_dn_prefetch_control: XDnsPrefetchControl::default(),
			x_download_options: XDownloadOptions::Enable,
			x_content_type_options: XContentTypeOptions::Enable,
			strict_transport_security: Some(StrictTransportSecurity::default()),
		}
	}
}
