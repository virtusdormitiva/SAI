use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{self, PgPool};
use thiserror::Error;
use uuid::Uuid;

use crate::models::user::{Role, User};
use crate::utils::pagination::{PaginationOptions, PaginationResponse};

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Error de la base de datos: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Usuario no encontrado")]
    NotFound,
    #[error("Error en la petición: {0}")]
    BadRequest(String),
    #[error("Error de validación: {0}")]
    ValidationError(String),
    #[error("Error interno del servidor: {0}")]
    InternalServerError(String),
}

#[derive(Debug, Error)]
pub enum CreateUserError {
    #[error("Usuario con este email ya existe")]
    EmailAlreadyExists,
    #[error("Usuario con este nombre de usuario ya existe")]
    UsernameAlreadyExists,
    #[error("Error de validación: {0}")]
    ValidationError(String),
    #[error("Error interno: {0}")]
    InternalError(String),
}

#[derive(Debug, Error)]
pub enum UpdateUserError {
    #[error("Usuario no encontrado")]
    NotFound,
    #[error("Usuario con este email ya existe")]
    EmailAlreadyExists,
    #[error("Usuario con este nombre de usuario ya existe")]
    UsernameAlreadyExists,
    #[error("Error de validación: {0}")]
    ValidationError(String),
    #[error("Error interno: {0}")]
    InternalError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub role: Role,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub role: Role,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role: Option<Role>,
    pub is_active: Option<bool>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            role: user.role,
            is_active: user.is_active,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

pub struct UserService;

impl UserService {
    pub async fn get_all_users(
        pool: &PgPool,
        pagination: PaginationOptions,
    ) -> Result<PaginationResponse<UserResponse>, ServiceError> {
        let offset = (pagination.page - 1) * pagination.per_page;
        
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            pagination.per_page as i64,
            offset as i64
        )
        .fetch_all(pool)
        .await?;

        let total_count = sqlx::query!(
            r#"
            SELECT COUNT(*) as count FROM users
            "#
        )
        .fetch_one(pool)
        .await?
        .count
        .unwrap_or(0) as usize;

        let total_pages = (total_count as f64 / pagination.per_page as f64).ceil() as usize;
        
        Ok(PaginationResponse {
            data: users.into_iter().map(UserResponse::from).collect(),
            pagination: pagination.to_response(total_count, total_pages),
        })
    }

    pub async fn get_user_by_id(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<UserResponse, ServiceError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users WHERE id = $1
            "#,
            user_id
        )
        .fetch_optional(pool)
        .await?
        .ok_or(ServiceError::NotFound)?;

        Ok(UserResponse::from(user))
    }

    pub async fn get_user_by_username(
        pool: &PgPool,
        username: &str,
    ) -> Result<UserResponse, ServiceError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users WHERE username = $1
            "#,
            username
        )
        .fetch_optional(pool)
        .await?
        .ok_or(ServiceError::NotFound)?;

        Ok(UserResponse::from(user))
    }

    pub async fn get_user_by_email(
        pool: &PgPool,
        email: &str,
    ) -> Result<UserResponse, ServiceError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users WHERE email = $1
            "#,
            email
        )
        .fetch_optional(pool)
        .await?
        .ok_or(ServiceError::NotFound)?;

        Ok(UserResponse::from(user))
    }

    pub async fn create_user(
        pool: &PgPool,
        user_data: CreateUserRequest,
    ) -> Result<UserResponse, CreateUserError> {
        // Validate username and email uniqueness
        let existing_username = sqlx::query!(
            r#"
            SELECT id FROM users WHERE username = $1
            "#,
            user_data.username
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| CreateUserError::InternalError(e.to_string()))?;

        if existing_username.is_some() {
            return Err(CreateUserError::UsernameAlreadyExists);
        }

        let existing_email = sqlx::query!(
            r#"
            SELECT id FROM users WHERE email = $1
            "#,
            user_data.email
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| CreateUserError::InternalError(e.to_string()))?;

        if existing_email.is_some() {
            return Err(CreateUserError::EmailAlreadyExists);
        }

        // Hash the password - in a real implementation, you would use argon2 or bcrypt
        // For this example, I'll use a simple placeholder
        let password_hash = format!("hashed_{}", user_data.password);

        let now = Utc::now();
        let user_id = Uuid::new_v4();

        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (
                id, username, email, password_hash, 
                first_name, last_name, role, is_active, 
                created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING *
            "#,
            user_id,
            user_data.username,
            user_data.email,
            password_hash,
            user_data.first_name,
            user_data.last_name,
            user_data.role as Role,
            true,
            now,
            now
        )
        .fetch_one(pool)
        .await
        .map_err(|e| CreateUserError::InternalError(e.to_string()))?;

        Ok(UserResponse::from(user))
    }

    pub async fn update_user(
        pool: &PgPool,
        user_id: Uuid,
        update_data: UpdateUserRequest,
    ) -> Result<UserResponse, UpdateUserError> {
        // Check if the user exists
        let existing_user = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users WHERE id = $1
            "#,
            user_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| UpdateUserError::InternalError(e.to_string()))?
        .ok_or(UpdateUserError::NotFound)?;

        // Check username uniqueness if it's being updated
        if let Some(username) = &update_data.username {
            if username != &existing_user.username {
                let existing_username = sqlx::query!(
                    r#"
                    SELECT id FROM users WHERE username = $1 AND id != $2
                    "#,
                    username,
                    user_id
                )
                .fetch_optional(pool)
                .await
                .map_err(|e| UpdateUserError::InternalError(e.to_string()))?;

                if existing_username.is_some() {
                    return Err(UpdateUserError::UsernameAlreadyExists);
                }
            }
        }

        // Check email uniqueness if it's being updated
        if let Some(email) = &update_data.email {
            if email != &existing_user.email {
                let existing_email = sqlx::query!(
                    r#"
                    SELECT id FROM users WHERE email = $1 AND id != $2
                    "#,
                    email,
                    user_id
                )
                .fetch_optional(pool)
                .await
                .map_err(|e| UpdateUserError::InternalError(e.to_string()))?;

                if existing_email.is_some() {
                    return Err(UpdateUserError::EmailAlreadyExists);
                }
            }
        }

        // Process password if it's being updated
        let password_hash = if let Some(password) = &update_data.password {
            // In a real app, hash the password with a proper algorithm
            Some(format!("hashed_{}", password))
        } else {
            None
        };

        // Update the user record
        let now = Utc::now();
        let updated_user = sqlx::query_as!(
            User,
            r#"
            UPDATE users SET
                username = COALESCE($1, username),
                email = COALESCE($2, email),
                password_hash = COALESCE($3, password_hash),
                first_name = COALESCE($4, first_name),
                last_name = COALESCE($5, last_name),
                role = COALESCE($6, role),
                is_active = COALESCE($7, is_active),
                updated_at = $8
            WHERE id = $9
            RETURNING *
            "#,
            update_data.username,
            update_data.email,
            password_hash,
            update_data.first_name,
            update_data.last_name,
            update_data.role as Option<Role>,
            update_data.is_active,
            now,
            user_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| UpdateUserError::InternalError(e.to_string()))?;

        Ok(UserResponse::from(updated_user))
    }

    pub async fn delete_user(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<(), ServiceError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM users WHERE id = $1
            "#,
            user_id
        )
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(ServiceError::NotFound);
        }

        Ok(())
    }
}

