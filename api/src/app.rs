use actix::prelude::*;
use actix_web::{http::Method, middleware, App, middleware::identity::{IdentityService, CookieIdentityPolicy}, middleware::cors::{Cors}};
use db::DbExecutor;
use api::{register_user, get_reviews, get_full_reviews, create_full_review, update_full_review, upload,
          delete_review, change_review_status, login, logout, get_user, get_images, get_full_images, create_full_image,
    update_full_image, delete_image, change_image_status

};
use chrono::Duration;
use middlewares::Auth;
use middlewares::AuthRoute;

pub struct AppState {
    pub db: Addr<DbExecutor>,
}

// helper function to create and returns the app after mounting all routes/resources
pub fn create_app(db: Addr<DbExecutor>) -> App<AppState> {

    // secret is a random 32 character long base 64 string
    let secret: String = ::std::env::var("SECRET_KEY").unwrap_or_else(|_| "0".repeat(32));
    let domain: String = ::std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

    App::with_state(AppState {db})
        // setup builtin logger to get nice logging for each request
        .middleware(middleware::Logger::new("\"%r\" %s %b %Dms"))
        .middleware(
            IdentityService::new(CookieIdentityPolicy::new(secret.as_bytes())
                .name("auth")
                .path("/")
                .domain(domain.as_str())
                .max_age(Duration::days(1))
                .secure(false),
            )
        )
        .middleware(Auth {
            routes: vec![
                AuthRoute {name: "auth", method: Method::GET, roles: vec!["Admin"]},
                AuthRoute {name: "background", method: Method::GET, roles: vec!["Admin"]},
                AuthRoute {name: "background", method: Method::POST, roles: vec!["Admin"]},
                AuthRoute {name: "backgroundWithId", method: Method::POST, roles: vec!["Admin"]},
                AuthRoute {name: "backgroundWithId", method: Method::PATCH, roles: vec!["Admin"]},
                AuthRoute {name: "backgroundWithId", method: Method::DELETE, roles: vec!["Admin"]},
                AuthRoute {name: "fullReview", method: Method::GET, roles: vec!["Admin"]},
                AuthRoute {name: "fullReview", method: Method::POST, roles: vec!["Admin"]},
                AuthRoute {name: "fullReviewWithId", method: Method::POST, roles: vec!["Admin"]},
                AuthRoute {name: "fullReviewWithId", method: Method::PATCH, roles: vec!["Admin"]},
                AuthRoute {name: "fullReviewWithId", method: Method::PUT, roles: vec!["Admin"]},
                AuthRoute {name: "upload", method: Method::POST, roles: vec!["Admin"]},
            ]
        })
        .configure(|app| {
            Cors::for_app(app)
                .max_age(3600)
                .resource("/api/auth", |r| {
                    r.name("auth");
                    r.method(Method::POST).with(login);
                    r.method(Method::DELETE).with(logout);
                    r.method(Method::GET).with(get_user);
                })
                .resource("/api/register", |r| {
                    r.method(Method::POST).with(register_user);
                })
                .resource("/api/images", |r| {
                    r.method(Method::GET).with(get_images);
                })
                .resource("/api/reviews", |r| {
                    r.method(Method::GET).with(get_reviews);
                })
                // ADMIN IMAGE - BACKGROUND
                .resource("/api/background", |r| {
                    r.name("background");
                    r.method(Method::GET).with(get_full_images);
                    r.method(Method::POST).with(create_full_image);
                })

                .resource("/api/background/{id}", |r| {
                    r.name("backgroundWithId");
                    r.method(Method::DELETE).with(delete_image);
                    r.method(Method::PATCH).with(change_image_status);
                    r.method(Method::POST).with(update_full_image);
                })
                // ADMIN REVIEW
                .resource("/api/full-reviews", |r| {
                    r.name("fullReview");
                    r.method(Method::GET).with(get_full_reviews);
                    r.method(Method::POST).with(create_full_review);
                })

                .resource("/api/full-reviews/{id}", |r| {
                    r.name("fullReviewWithId");
                    r.method(Method::DELETE).with(delete_review);
                    r.method(Method::PATCH).with(change_review_status);
                    r.method(Method::POST).with(update_full_review);
                })
                .resource("/api/upload", |r| {
                    r.name("upload");
                    r.method(Method::POST).with(upload);
                })
                .register()
        })

}