use actix_web::web;

use crate::post::post_controller;

pub fn route(cfg: &mut web::ServiceConfig) {
	cfg.service(web::resource("").route(web::post().to(post_controller::create::execute)));
	cfg.service(
		web::resource("/{id}")
			.route(web::get().to(post_controller::get::execute))
			.route(web::patch().to(post_controller::update::execute))
			.route(web::delete().to(post_controller::delete::execute)),
	);
}
