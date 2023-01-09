use actix_web::web;

use crate::auth::auth_controller::get_tokens;

pub fn auth_route(cfg: &mut web::ServiceConfig) {
	cfg.service(web::resource("/code").route(web::post().to(get_tokens)));
}
