use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::user::User;
use crate::services::users::{CreateUserError, UpdateUserError, UserService};

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role: Option<String>,
    pub is_active: Option<bool>,
}

#[get("")]
async fn get_all_users(user_service: web::Data<UserService>) -> impl Responder {
    match user_service.get_all_users().await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => {
            log::error!("Failed to get all users: {}", err);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to get users".to_string(),
            })
        }
    }
}

#[get("/{id}")]
async fn get_user_by_id(
    path: web::Path<(Uuid,)>,
    user_service: web::Data<UserService>,
) -> impl Responder {
    let user_id = path.into_inner().0;

    match user_service.get_user_by_id(user_id).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json(ErrorResponse {
            error: format!("User with id {} not found", user_id),
        }),
        Err(err) => {
            log::error!("Failed to get user: {}", err);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to get user".to_string(),
            })
        }
    }
}

#[post("")]
async fn create_user(
    request: web::Json<CreateUserRequest>,
    user_service: web::Data<UserService>,
) -> impl Responder {
    match user_service
        .create_user(
            &request.username,
            &request.email,
            &request.password,
            &request.first_name,
            &request.last_name,
            &request.role,
        )
        .await
    {
        Ok(user) => HttpResponse::Created().json(user),
        Err(CreateUserError::UsernameAlreadyExists) => {
            HttpResponse::BadRequest().json(ErrorResponse {
                error: "Username already exists".to_string(),
            })
        }
        Err(CreateUserError::EmailAlreadyExists) => {
            HttpResponse::BadRequest().json(ErrorResponse {
                error: "Email already exists".to_string(),
            })
        }
        Err(CreateUserError::InvalidRole) => {
            HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid role".to_string(),
            })
        }
        Err(CreateUserError::DatabaseError(err)) => {
            log::error!("Failed to create user: {}", err);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to create user".to_string(),
            })
        }
    }
}

#[put("/{id}")]
async fn update_user(
    path: web::Path<(Uuid,)>,
    request: web::Json<UpdateUserRequest>,
    user_service: web::Data<UserService>,
) -> impl Responder {
    let user_id = path.into_inner().0;

    match user_service
        .update_user(
            user_id,
            request.username.as_deref(),
            request.email.as_deref(),
            request.password.as_deref(),
            request.first_name.as_deref(),
            request.last_name.as_deref(),
            request.role.as_deref(),
            request.is_active,
        )
        .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(UpdateUserError::UserNotFound) => {
            HttpResponse::NotFound().json(ErrorResponse {
                error: format!("User with id {} not found", user_id),
            })
        }
        Err(UpdateUserError::UsernameAlreadyExists) => {
            HttpResponse::BadRequest().json(ErrorResponse {
                error: "Username already exists".to_string(),
            })
        }
        Err(UpdateUserError::EmailAlreadyExists) => {
            HttpResponse::BadRequest().json(ErrorResponse {
                error: "Email already exists".to_string(),
            })
        }
        Err(UpdateUserError::InvalidRole) => {
            HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid role".to_string(),
            })
        }
        Err(UpdateUserError::DatabaseError(err)) => {
            log::error!("Failed to update user: {}", err);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to update user".to_string(),
            })
        }
    }
}

#[delete("/{id}")]
async fn delete_user(
    path: web::Path<(Uuid,)>,
    user_service: web::Data<UserService>,
) -> impl Responder {
    let user_id = path.into_inner().0;

    match user_service.delete_user(user_id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json(ErrorResponse {
            error: format!("User with id {} not found", user_id),
        }),
        Err(err) => {
            log::error!("Failed to delete user: {}", err);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to delete user".to_string(),
            })
        }
    }
}

pub fn routes() -> web::Scope {
    web::Scope::new("/users")
        .service(get_all_users)
        .service(get_user_by_id)
        .service(create_user)
        .service(update_user)
        .service(delete_user)
}

