use actix_web::web;

use crate::post_category::post_category_controller;

pub fn route(cfg: &mut web::ServiceConfig) {
	cfg.service(
		web::resource("/").route(web::post().to(post_category_controller::create::execute)),
	);
}
