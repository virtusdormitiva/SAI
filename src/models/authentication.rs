use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPool, types::Uuid, Error as SqlxError};
use uuid::Uuid as UuidLib;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Authentication {
    pub id: Uuid,
    pub user_id: Uuid,
    pub password_hash: String,
    pub reset_token: Option<String>,
    pub reset_token_expires: Option<DateTime<Utc>>,
    pub token_version: i32,
    pub last_login: Option<DateTime<Utc>>,
    pub is_locked: bool,
    pub failed_attempts: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct NewAuthentication {
    pub user_id: Uuid,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthenticationUpdate {
    pub password: Option<String>,
    pub reset_token: Option<String>,
    pub reset_token_expires: Option<DateTime<Utc>>,
    pub token_version: Option<i32>,
    pub last_login: Option<DateTime<Utc>>,
    pub is_locked: Option<bool>,
    pub failed_attempts: Option<i32>,
}

impl Authentication {
    /// Create a new authentication record
    pub async fn create(pool: &PgPool, new_auth: NewAuthentication) -> Result<Self, SqlxError> {
        let password_hash = bcrypt::hash(&new_auth.password, bcrypt::DEFAULT_COST)
            .map_err(|e| SqlxError::Protocol(format!("Failed to hash password: {}", e)))?;

        let auth = sqlx::query_as!(
            Authentication,
            r#"
            INSERT INTO authentications (
                user_id, password_hash, token_version, is_locked, failed_attempts
            )
            VALUES ($1, $2, 0, false, 0)
            RETURNING id, user_id, password_hash, reset_token, reset_token_expires, token_version, 
                      last_login, is_locked, failed_attempts, created_at, updated_at
            "#,
            new_auth.user_id,
            password_hash,
        )
        .fetch_one(pool)
        .await?;

        Ok(auth)
    }

    /// Find an authentication record by user_id
    pub async fn find_by_user_id(pool: &PgPool, user_id: Uuid) -> Result<Self, SqlxError> {
        let auth = sqlx::query_as!(
            Authentication,
            r#"
            SELECT * FROM authentications WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(auth)
    }

    /// Find an authentication record by reset token
    pub async fn find_by_reset_token(
        pool: &PgPool,
        reset_token: &str,
    ) -> Result<Self, SqlxError> {
        let auth = sqlx::query_as!(
            Authentication,
            r#"
            SELECT * FROM authentications 
            WHERE reset_token = $1 AND reset_token_expires > now()
            "#,
            reset_token
        )
        .fetch_one(pool)
        .await?;

        Ok(auth)
    }

    /// Update an authentication record
    pub async fn update(
        &self,
        pool: &PgPool,
        update: AuthenticationUpdate,
    ) -> Result<Self, SqlxError> {
        let password_hash = match update.password {
            Some(password) => Some(
                bcrypt::hash(&password, bcrypt::DEFAULT_COST)
                    .map_err(|e| SqlxError::Protocol(format!("Failed to hash password: {}", e)))?,
            ),
            None => None,
        };

        let auth = sqlx::query_as!(
            Authentication,
            r#"
            UPDATE authentications
            SET 
                password_hash = COALESCE($1, password_hash),
                reset_token = COALESCE($2, reset_token),
                reset_token_expires = COALESCE($3, reset_token_expires),
                token_version = COALESCE($4, token_version),
                last_login = COALESCE($5, last_login),
                is_locked = COALESCE($6, is_locked),
                failed_attempts = COALESCE($7, failed_attempts),
                updated_at = now()
            WHERE id = $8
            RETURNING id, user_id, password_hash, reset_token, reset_token_expires, token_version, 
                      last_login, is_locked, failed_attempts, created_at, updated_at
            "#,
            password_hash,
            update.reset_token,
            update.reset_token_expires,
            update.token_version,
            update.last_login,
            update.is_locked,
            update.failed_attempts,
            self.id
        )
        .fetch_one(pool)
        .await?;

        Ok(auth)
    }

    /// Delete an authentication record
    pub async fn delete(self, pool: &PgPool) -> Result<(), SqlxError> {
        sqlx::query!(
            r#"
            DELETE FROM authentications WHERE id = $1
            "#,
            self.id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Record a login attempt - updates last_login if successful, or increments failed_attempts if failed
    pub async fn record_login_attempt(
        &self,
        pool: &PgPool,
        success: bool,
    ) -> Result<Self, SqlxError> {
        if success {
            let auth = sqlx::query_as!(
                Authentication,
                r#"
                UPDATE authentications
                SET 
                    last_login = now(),
                    failed_attempts = 0,
                    is_locked = false,
                    updated_at = now()
                WHERE id = $1
                RETURNING id, user_id, password_hash, reset_token, reset_token_expires, token_version, 
                          last_login, is_locked, failed_attempts, created_at, updated_at
                "#,
                self.id
            )
            .fetch_one(pool)
            .await?;

            Ok(auth)
        } else {
            // Set the maximum failed attempts before locking account
            const MAX_FAILED_ATTEMPTS: i32 = 5;
            
            let new_failed_attempts = self.failed_attempts + 1;
            let is_locked = new_failed_attempts >= MAX_FAILED_ATTEMPTS;

            let auth = sqlx::query_as!(
                Authentication,
                r#"
                UPDATE authentications
                SET 
                    failed_attempts = $1,
                    is_locked = $2,
                    updated_at = now()
                WHERE id = $3
                RETURNING id, user_id, password_hash, reset_token, reset_token_expires, token_version, 
                          last_login, is_locked, failed_attempts, created_at, updated_at
                "#,
                new_failed_attempts,
                is_locked,
                self.id
            )
            .fetch_one(pool)
            .await?;

            Ok(auth)
        }
    }

    /// Generate a password reset token
    pub async fn generate_reset_token(&self, pool: &PgPool) -> Result<String, SqlxError> {
        // Generate a random token
        let reset_token = UuidLib::new_v4().to_string();
        
        // Set token to expire in 24 hours
        let expires = Utc::now() + chrono::Duration::hours(24);

        sqlx::query!(
            r#"
            UPDATE authentications
            SET 
                reset_token = $1,
                reset_token_expires = $2,
                updated_at = now()
            WHERE id = $3
            "#,
            reset_token,
            expires,
            self.id
        )
        .execute(pool)
        .await?;

        Ok(reset_token)
    }

    /// Clear the reset token
    pub async fn clear_reset_token(&self, pool: &PgPool) -> Result<(), SqlxError> {
        sqlx::query!(
            r#"
            UPDATE authentications
            SET 
                reset_token = NULL,
                reset_token_expires = NULL,
                updated_at = now()
            WHERE id = $1
            "#,
            self.id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Increment token version (invalidates all existing tokens)
    pub async fn increment_token_version(&self, pool: &PgPool) -> Result<Self, SqlxError> {
        let auth = sqlx::query_as!(
            Authentication,
            r#"
            UPDATE authentications
            SET 
                token_version = token_version + 1,
                updated_at = now()
            WHERE id = $1
            RETURNING id, user_id, password_hash, reset_token, reset_token_expires, token_version, 
                      last_login, is_locked, failed_attempts, created_at, updated_at
            "#,
            self.id
        )
        .fetch_one(pool)
        .await?;

        Ok(auth)
    }

    /// Verify a password
    pub fn verify_password(&self, password: &str) -> bool {
        bcrypt::verify(password, &self.password_hash).unwrap_or(false)
    }

    /// Check if the account is locked
    pub fn is_account_locked(&self) -> bool {
        self.is_locked
    }
}

