use actix_web::web;
use crate::user::user_controller::{ get_me, update_user };

pub fn user_route(cfg: &mut web::ServiceConfig) {
	cfg.service(web::resource("/me").route(web::get().to(get_me)));
	cfg.service(web::resource("/{id}").route(web::patch().to(update_user)));
}
