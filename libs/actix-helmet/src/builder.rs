use crate::{inner::Inner, middleware::HelmetMiddleware, policies::StrictTransportSecurity};
use actix_web::{
	dev::{Service, ServiceRequest, ServiceResponse, Transform},
	Error,
};
use std::{
	future::{ready, Ready},
	rc::Rc,
};

#[non_exhaustive]
#[derive(Clone, Default)]
pub struct Helmet {
	inner: Rc<Inner>,
}

impl Helmet {
	pub fn enable_strict_transport_security(
		mut self,
		max_age: u32,
		include_sub_domains: bool,
		preload: bool,
	) -> Helmet {
		if let Some(helmet) = Rc::get_mut(&mut self.inner) {
			let strict_transport_security = StrictTransportSecurity {
				max_age,
				include_sub_domains,
				preload,
			};
			helmet.strict_transport_security = Some(strict_transport_security);
		}
		self
	}

	pub fn disable_strict_transport_security(mut self) -> Helmet {
		if let Some(helmet) = Rc::get_mut(&mut self.inner) {
			helmet.strict_transport_security = None;
		}
		self
	}
}

impl<S, B> Transform<S, ServiceRequest> for Helmet
where
	S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
	S::Future: 'static,
	B: 'static,
{
	type Response = ServiceResponse<B>;
	type Error = Error;
	type InitError = ();
	type Transform = HelmetMiddleware<S>;
	type Future = Ready<Result<Self::Transform, Self::InitError>>;

	fn new_transform(&self, service: S) -> Self::Future {
		let middleware = HelmetMiddleware {
			service,
			inner: self.inner.clone(),
		};
		ready(Ok(middleware))
	}
}
