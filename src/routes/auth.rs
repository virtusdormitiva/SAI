use actix_web::{
    web, HttpResponse, Responder, Scope, 
    post, get, put, delete, HttpRequest,
    cookie::{Cookie, SameSite},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::Mutex;
use std::collections::HashMap;

/// Authentication service for SAI system
///
/// Provides routes for user authentication, JWT token management,
/// and password reset functionality.
pub struct Auth {
    token_blacklist: Mutex<HashMap<String, chrono::DateTime<Utc>>>,
}

/// JWT Claims structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID)
    sub: String,
    /// User role (admin, teacher, student, etc.)
    role: String,
    /// Expiration time (as UTC timestamp)
    exp: usize,
    /// Issued at (as UTC timestamp)
    iat: usize,
}

/// Login request data
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

/// Registration request data
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    username: String,
    email: String,
    password: String,
    confirm_password: String,
}

/// Password reset request data
#[derive(Debug, Deserialize)]
pub struct PasswordResetRequest {
    email: String,
}

/// Password update request data
#[derive(Debug, Deserialize)]
pub struct PasswordUpdateRequest {
    token: String,
    new_password: String,
    confirm_password: String,
}

/// Token refresh request data
#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    refresh_token: String,
}

/// Authentication response
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    token: String,
    refresh_token: String,
    user_id: String,
    role: String,
}

/// Error response
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    error: String,
    message: String,
}

/// Token type for validation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    /// Access token used for API authorization
    Access,
    /// Refresh token used to get new access tokens
    Refresh,
}

impl Auth {
    /// Create a new Auth service instance
    pub fn new() -> Self {
        Auth {
            token_blacklist: Mutex::new(HashMap::new()),
        }
    }

    /// Cleanup expired tokens from the blacklist
    fn cleanup_blacklist(&self) {
        let mut blacklist = self.token_blacklist.lock().unwrap();
        let now = Utc::now();
        blacklist.retain(|_, exp| *exp > now);
    }

    /// Generate a JWT token for a user
    fn generate_token(&self, user_id: &str, role: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let exp = Utc::now() + Duration::hours(1);
        let claims = Claims {
            sub: user_id.to_string(),
            role: role.to_string(),
            exp: exp.timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(
                std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string()).as_ref()
            ),
        )
    }

    /// Generate a refresh token
    fn generate_refresh_token(&self) -> String {
        Uuid::new_v4().to_string()
    }

    /// Validate a JWT token
    ///
    /// Static method that can be called without an Auth instance
    /// 
    /// # Examples
    ///
    /// ```
    /// let result = Auth::validate_token(token, TokenType::Access);
    /// ```
    pub fn validate_token(token: &str, token_type: TokenType) -> Result<Claims, jsonwebtoken::errors::Error> {
        let mut validation = Validation::new(Algorithm::HS256);
        
        // Apply different validation rules based on token type
        match token_type {
            TokenType::Access => {
                // Standard validation for access tokens
                validation.validate_exp = true;
            },
            TokenType::Refresh => {
                // Refresh tokens might have different validation rules
                validation.validate_exp = true;
                // Additional validation for refresh tokens could be added here
            }
        }
        
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(
                std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string()).as_ref()
            ),
            &validation,
        )?;

        Ok(token_data.claims)
    }

    /// Handle login requests
    async fn login(&self, req: web::Json<LoginRequest>) -> HttpResponse {
        // In a real implementation, validate against database
        // This is a placeholder for demonstration
        if req.username == "admin" && req.password == "password" {
            match self.generate_token("1", "admin") {
                Ok(token) => {
                    let refresh_token = self.generate_refresh_token();
                    
                    // Create a cookie for the token
                    let cookie = Cookie::build("auth_token", token.clone())
                        .path("/")
                        .secure(true)
                        .http_only(true)
                        .same_site(SameSite::Strict)
                        .max_age(time::Duration::hours(1))
                        .finish();

                    HttpResponse::Ok()
                        .cookie(cookie)
                        .json(AuthResponse {
                            token,
                            refresh_token,
                            user_id: "1".to_string(),
                            role: "admin".to_string(),
                        })
                } 
                Err(_) => {
                    HttpResponse::InternalServerError().json(ErrorResponse {
                        error: "token_generation_failed".to_string(),
                        message: "Failed to generate authentication token".to_string(),
                    })
                }
            }
        } else {
            HttpResponse::Unauthorized().json(ErrorResponse {
                error: "invalid_credentials".to_string(),
                message: "Invalid username or password".to_string(),
            })
        }
    }

    /// Handle register requests
    async fn register(&self, req: web::Json<RegisterRequest>) -> HttpResponse {
        // Validate request
        if req.password != req.confirm_password {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "password_mismatch".to_string(),
                message: "Passwords do not match".to_string(),
            });
        }

        // In a real implementation, check if user exists and save to database
        // This is a placeholder for demonstration
        let user_id = Uuid::new_v4().to_string();
        
        match self.generate_token(&user_id, "user") {
            Ok(token) => {
                let refresh_token = self.generate_refresh_token();
                
                HttpResponse::Created().json(AuthResponse {
                    token,
                    refresh_token,
                    user_id,
                    role: "user".to_string(),
                })
            }
            Err(_) => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "token_generation_failed".to_string(),
                    message: "Failed to generate authentication token".to_string(),
                })
            }
        }
    }

    /// Handle logout requests
    async fn logout(&self, req: HttpRequest) -> HttpResponse {
        // Extract token from Authorization header or cookie
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = auth_str[7..].to_string();
                    
                    // Add token to blacklist
                    let mut blacklist = self.token_blacklist.lock().unwrap();
                    blacklist.insert(token, Utc::now() + Duration::hours(24));
                    
                    // Clean up expired tokens occasionally
                    if blacklist.len() % 100 == 0 {
                        drop(blacklist);
                        self.cleanup_blacklist();
                    }
                }
            }
        }

        // Remove the auth cookie
        let cookie = Cookie::build("auth_token", "")
            .path("/")
            .max_age(time::Duration::seconds(0))
            .finish();

        HttpResponse::Ok()
            .cookie(cookie)
            .json(serde_json::json!({
                "message": "Successfully logged out"
            }))
    }

    /// Handle password reset requests
    async fn request_password_reset(&self, req: web::Json<PasswordResetRequest>) -> HttpResponse {
        // In a real implementation, this would:
        // 1. Check if email exists
        // 2. Generate a reset token
        // 3. Store token with expiration
        // 4. Send email with reset link
        
        // This is a placeholder implementation
        HttpResponse::Ok().json(serde_json::json!({
            "message": "Password reset instructions sent to email if it exists in our system"
        }))
    }

    /// Handle password update after reset
    async fn update_password(&self, req: web::Json<PasswordUpdateRequest>) -> HttpResponse {
        // Validate request
        if req.new_password != req.confirm_password {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "password_mismatch".to_string(),
                message: "Passwords do not match".to_string(),
            });
        }

        // In a real implementation, this would:
        // 1. Validate the reset token
        // 2. Check if token is expired
        // 3. Update the user's password
        // 4. Revoke the reset token
        
        // This is a placeholder implementation
        HttpResponse::Ok().json(serde_json::json!({
            "message": "Password successfully updated"
        }))
    }

    /// Handle token refresh requests
    async fn refresh_token(&self, req: web::Json<RefreshTokenRequest>) -> HttpResponse {
        // In a real implementation, validate the refresh token against stored tokens
        // This is a placeholder for demonstration
        
        // Validate refresh token
        // In a real implementation, this would validate against stored refresh tokens
        // For now, we'll skip this step and just generate a new token
        
        // Generate a new token
        match self.generate_token("1", "admin") {
            Ok(token) => {
                let refresh_token = self.generate_refresh_token();
                
                HttpResponse::Ok().json(AuthResponse {
                    token,
                    refresh_token,
                    user_id: "1".to_string(),
                    role: "admin".to_string(),
                })
            }
            Err(_) => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "token_generation_failed".to_string(),
                    message: "Failed to generate authentication token".to_string(),
                })
            }
        }
    }
}

/// Configure authentication routes for Actix-web
/// 
/// This function sets up all authentication endpoints:
/// - POST /auth/login - Authenticates a user and returns tokens
/// - POST /auth/register - Creates a new user account
/// - POST /auth/logout - Invalidates the current session
/// - POST /auth/password-reset - Initiates password reset process
/// - PUT /auth/password-update - Completes password reset with a token
/// - POST /auth/refresh - Refreshes an expired access token
///
/// Returns a configured Scope that can be added to an Actix-web App
pub fn routes() -> Scope {
    let auth = web::Data::new(Auth::new());
    
    web::scope("/auth")
        .app_data(auth.clone())
        .route("/login", post().to(|payload: web::Json<LoginRequest>, auth: web::Data<Auth>| 
            auth.login(payload)))
        .route("/register", post().to(|payload: web::Json<RegisterRequest>, auth: web::Data<Auth>| 
            auth.register(payload)))
        .route("/logout", post().to(|req: HttpRequest, auth: web::Data<Auth>| 
            auth.logout(req)))
        .route("/password-reset", post().to(|payload: web::Json<PasswordResetRequest>, auth: web::Data<Auth>| 
            auth.request_password_reset(payload)))
        .route("/password-update", put().to(|payload: web::Json<PasswordUpdateRequest>, auth: web::Data<Auth>| 
            auth.update_password(payload)))
        .route("/refresh", post().to(|payload: web::Json<RefreshTokenRequest>, auth: web::Data<Auth>| 
            auth.refresh_token(payload)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};
    
    #[actix_rt::test]
    async fn test_login_success() {
        let auth = Auth::new();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(auth))
                .service(routes())
        ).await;
        
        let req = test::TestRequest::post()
            .uri("/auth/login")
            .set_json(&LoginRequest {
                username: "admin".to_string(),
                password: "password".to_string(),
            })
            .to_request();
            
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
    
    #[actix_rt::test]
    async fn test_login_failure() {
        let auth = Auth::new();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(auth))
                .service(routes())
        ).await;
        
        let req = test::TestRequest::post()
            .uri("/auth/login")
            .set_json(&LoginRequest {
                username: "admin".to_string(),
                password: "wrong".to_string(),
            })
            .to_request();
            
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 401);
    }
    
    #[test]
    fn test_token_type_enum() {
        assert_ne!(TokenType::Access, TokenType::Refresh);
    }
}

