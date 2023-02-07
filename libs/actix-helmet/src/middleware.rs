use crate::{
	inner::Inner,
	policies::{PolicyTrait, XContentTypeOptions, XDownloadOptions},
};
use actix_web::{
	dev::{forward_ready, Service, ServiceRequest, ServiceResponse},
	http::header::{self, HeaderName, HeaderValue},
	Error,
};
use futures_util::future::LocalBoxFuture;
use std::rc::Rc;

pub struct HelmetMiddleware<S> {
	pub service: S,
	pub inner: Rc<Inner>,
}

impl<S> HelmetMiddleware<S> {
	fn augment_response<B>(mut res: ServiceResponse<B>, inner: &Inner) -> ServiceResponse<B> {
		let header_map = res.headers_mut();
		header_map.insert(
			header::X_XSS_PROTECTION,
			inner.x_xss_protection.get_header_value(),
		);
		header_map.insert(
			header::REFERRER_POLICY,
			inner.referrer_policy.get_header_value(),
		);
		header_map.insert(
			header::X_FRAME_OPTIONS,
			inner.x_frame_options.get_header_value(),
		);
		if inner.x_download_options == XDownloadOptions::Enable {
			header_map.insert(
				HeaderName::from_lowercase(b"x-download-options").unwrap(),
				HeaderValue::from_static("noopen"),
			);
		}
		header_map.insert(
			HeaderName::from_lowercase(b"x-permitted-cross-domain-policies").unwrap(),
			inner.x_permitted_cross_domain_policies.get_header_value(),
		);
		header_map.insert(
			header::X_DNS_PREFETCH_CONTROL,
			inner.x_dn_prefetch_control.get_header_value(),
		);
		if inner.x_content_type_options == XContentTypeOptions::Enable {
			header_map.insert(
				header::X_CONTENT_TYPE_OPTIONS,
				HeaderValue::from_static("nosniff"),
			);
		}
		if let Some(strict_transport_security) = inner.strict_transport_security.to_owned() {
			header_map.insert(
				header::STRICT_TRANSPORT_SECURITY,
				strict_transport_security.get_header_value(),
			);
		}
		res
	}
}

impl<S, B> Service<ServiceRequest> for HelmetMiddleware<S>
where
	S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
	S::Future: 'static,
	B: 'static,
{
	type Response = ServiceResponse<B>;
	type Error = Error;
	type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

	fn call(&self, req: ServiceRequest) -> Self::Future {
		let fut = self.service.call(req);
		let inner = Rc::clone(&self.inner);
		Box::pin(async move {
			let res = fut.await?;
			Ok(Self::augment_response(res, &inner))
		})
	}

	forward_ready!(service);
}
