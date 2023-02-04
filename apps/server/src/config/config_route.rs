use actix_web::web;

use crate::config::config_controller;

pub fn route(cfg: &mut web::ServiceConfig) {
	cfg.service(web::resource("").route(web::get().to(config_controller::get_config::execute)));
}
