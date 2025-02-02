use super::handlers;
use actix_web::{web, Scope};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        Scope::new("/api/v1").service(
            web::scope("/links")
                .route("/create-short-link", web::post().to(handlers::create_short_link))
                .route("/redirect/{slug}", web::get().to(handlers::redirect))
                .route("/{slug}/stats", web::get().to(handlers::get_stats)),
        ),
    );
}
