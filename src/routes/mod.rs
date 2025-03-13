//! Routes module for the SAI API.
//! This module defines all the HTTP routes and handlers for the application.

use actix_web::{web, Scope};

// Import submodules
mod users;
mod students;
mod courses;
mod teachers;
mod attendance;
mod grades;
mod schedules;
mod reports;
mod auth;

/// Configure all API routes
pub fn configure() -> Scope {
    web::scope("/api")
        .service(auth::routes())
        .service(users::routes())
        .service(students::routes())
        .service(teachers::routes())
        .service(courses::routes())
        .service(attendance::routes())
        .service(grades::routes())
        .service(schedules::routes())
        .service(reports::routes())
}

/// Configure health check and system status routes
pub fn configure_system_routes() -> Scope {
    web::scope("/system")
        .route("/health", web::get().to(health_check))
        .route("/status", web::get().to(system_status))
}

/// Simple health check handler
async fn health_check() -> &'static str {
    "OK"
}

/// System status handler
async fn system_status() -> web::Json<serde_json::Value> {
    web::Json(serde_json::json!({
        "status": "running",
        "version": env!("CARGO_PKG_VERSION"),
        "environment": std::env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".to_string())
    }))
}

// Public re-exports for easier module usage
pub use auth::Auth;
pub use users::User;
pub use students::Student;
pub use courses::Course;

