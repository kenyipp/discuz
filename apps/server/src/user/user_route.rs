use actix_web::web;

use crate::user::user_controller;

pub fn user_route(cfg: &mut web::ServiceConfig) {
	cfg.service(web::resource("/me").route(web::get().to(user_controller::get_me::execute)));
	cfg.service(
		web::resource("/{id}").route(web::patch().to(user_controller::update_user::execute)),
	);
}
