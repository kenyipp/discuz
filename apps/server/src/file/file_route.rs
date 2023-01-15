use actix_web::web;

use crate::file::file_controller::get_upload_url;

pub fn route(cfg: &mut web::ServiceConfig) {
	cfg.service(web::resource("/upload").route(web::post().to(get_upload_url::execute)));
}
