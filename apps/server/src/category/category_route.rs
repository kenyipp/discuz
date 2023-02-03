use actix_web::web;

use crate::category::category_controller;

pub fn route(cfg: &mut web::ServiceConfig) {
	cfg.service(
		web::resource("")
			.route(web::get().to(category_controller::list::execute))
			.route(web::post().to(category_controller::create::execute)),
	);
}
