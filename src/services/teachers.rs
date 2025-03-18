use actix_web::{http::StatusCode, web, HttpResponse};
use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{
    teacher::{CreateTeacherDto, Teacher, TeacherFilter, UpdateTeacherDto, TeacherWithUserData},
    TeacherStatus,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTeacherRequest {
    pub user_id: Uuid,
    pub professional_id: String,
    pub specialization: String,
    pub hire_date: NaiveDate,
    pub education_level: String,
    pub subjects: Vec<String>,
    pub status: TeacherStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTeacherRequest {
    pub professional_id: Option<String>,
    pub specialization: Option<String>,
    pub hire_date: Option<NaiveDate>,
    pub education_level: Option<String>,
    pub subjects: Option<Vec<String>>,
    pub status: Option<TeacherStatus>,
}

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Teacher not found")]
    NotFound,
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Internal server error: {0}")]
    InternalServerError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
}

// Custom error types for teacher operations
#[derive(Debug, thiserror::Error)]
pub enum CreateTeacherError {
    #[error("Professional ID is required")]
    MissingProfessionalId,
    #[error("Specialization is required")]
    MissingSpecialization,
    #[error("Education level is required")]
    MissingEducationLevel,
    #[error("Hire date is required")]
    MissingHireDate,
    #[error("Invalid status")]
    InvalidStatus,
    #[error("Database error: {0}")]
    DatabaseError(String),
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateTeacherError {
    #[error("Teacher not found")]
    NotFound,
    #[error("Invalid professional ID format")]
    InvalidProfessionalId,
    #[error("Invalid specialization")]
    InvalidSpecialization,
    #[error("Invalid education level")]
    InvalidEducationLevel,
    #[error("Database error: {0}")]
    DatabaseError(String),
}

impl From<ServiceError> for HttpResponse {
    fn from(error: ServiceError) -> Self {
        match error {
            ServiceError::NotFound => HttpResponse::NotFound().json(error.to_string()),
            ServiceError::BadRequest(msg) => HttpResponse::BadRequest().json(msg),
            ServiceError::ValidationError(msg) => {
                HttpResponse::UnprocessableEntity().json(msg)
            }
            ServiceError::InternalServerError(msg) => {
                HttpResponse::InternalServerError().json(msg)
            }
        }
    }
}

pub struct TeacherService {
    pool: web::Data<PgPool>,
}

impl TeacherService {
    pub fn new(pool: web::Data<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn get_all_teachers(&self, filter: Option<TeacherFilter>, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<Teacher>, ServiceError> {
        let filter = filter.unwrap_or_default();
        
        Teacher::find_all(&self.pool, filter, limit, offset)
            .await
            .map_err(|e| ServiceError::InternalServerError(e.to_string()))
    }

    pub async fn get_teacher_by_id(&self, user_id: Uuid) -> Result<Teacher, ServiceError> {
        Teacher::find_by_user_id(&self.pool, user_id)
            .await
            .map_err(|e| ServiceError::InternalServerError(e.to_string()))
            .and_then(|maybe_teacher| {
                maybe_teacher.ok_or(ServiceError::NotFound)
            })
    }
    
    pub async fn get_teacher_with_user_data(&self, user_id: Uuid) -> Result<TeacherWithUserData, ServiceError> {
        Teacher::get_teacher_with_user_data(&self.pool, user_id)
            .await
            .map_err(|e| ServiceError::InternalServerError(e.to_string()))
            .and_then(|maybe_teacher| {
                maybe_teacher.ok_or(ServiceError::NotFound)
            })
    }
    
    pub async fn get_teacher_by_professional_id(&self, professional_id: &str) -> Result<Teacher, ServiceError> {
        Teacher::find_by_professional_id(&self.pool, professional_id)
            .await
            .map_err(|e| ServiceError::InternalServerError(e.to_string()))
            .and_then(|maybe_teacher| {
                maybe_teacher.ok_or(ServiceError::NotFound)
            })
    }

    pub async fn create_teacher(
        &self,
        request: CreateTeacherRequest,
    ) -> Result<Teacher, ServiceError> {
        // Validate the request
        Self::validate_create_teacher(&request)?;

        let dto = CreateTeacherDto {
            user_id: request.user_id,
            professional_id: request.professional_id,
            specialization: request.specialization,
            hire_date: request.hire_date,
            education_level: request.education_level,
            subjects: request.subjects,
            status: request.status,
        };

        Teacher::create(&self.pool, dto)
            .await
            .map_err(|e| ServiceError::InternalServerError(e.to_string()))
    }
    
    pub async fn update_teacher(
        &self,
        user_id: Uuid,
        request: UpdateTeacherRequest,
    ) -> Result<Teacher, ServiceError> {
        // First, check if the teacher exists
        self.get_teacher_by_id(user_id).await?;

        // Validate the request
        Self::validate_update_teacher(&request)?;

        let dto = UpdateTeacherDto {
            professional_id: request.professional_id,
            specialization: request.specialization,
            hire_date: request.hire_date,
            education_level: request.education_level,
            subjects: request.subjects,
            status: request.status,
        };

        Teacher::update(&self.pool, user_id, dto)
            .await
            .map_err(|e| ServiceError::InternalServerError(e.to_string()))
    }

    pub async fn delete_teacher(&self, user_id: Uuid) -> Result<(), ServiceError> {
        // First, check if the teacher exists
        self.get_teacher_by_id(user_id).await?;

        Teacher::delete(&self.pool, user_id)
            .await
            .map_err(|e| ServiceError::InternalServerError(e.to_string()))
            .map(|_| ())
    }

    // Helper methods for validation
    fn validate_create_teacher(request: &CreateTeacherRequest) -> Result<(), ServiceError> {
        if request.professional_id.is_empty() {
            return Err(ServiceError::ValidationError(
                "Professional ID cannot be empty".to_string(),
            ));
        }
        
        if request.specialization.is_empty() {
            return Err(ServiceError::ValidationError(
                "Specialization cannot be empty".to_string(),
            ));
        }
        
        if request.education_level.is_empty() {
            return Err(ServiceError::ValidationError(
                "Education level cannot be empty".to_string(),
            ));
        }
        
        if request.subjects.is_empty() {
            return Err(ServiceError::ValidationError(
                "Subjects list cannot be empty".to_string(),
            ));
        }

        // Add more validations as needed
        Ok(())
    }

    fn validate_update_teacher(request: &UpdateTeacherRequest) -> Result<(), ServiceError> {
        if let Some(ref professional_id) = request.professional_id {
            if professional_id.is_empty() {
                return Err(ServiceError::ValidationError(
                    "Professional ID cannot be empty".to_string(),
                ));
            }
        }
        
        if let Some(ref specialization) = request.specialization {
            if specialization.is_empty() {
                return Err(ServiceError::ValidationError(
                    "Specialization cannot be empty".to_string(),
                ));
            }
        }
        
        if let Some(ref education_level) = request.education_level {
            if education_level.is_empty() {
                return Err(ServiceError::ValidationError(
                    "Education level cannot be empty".to_string(),
                ));
            }
        }
        
        if let Some(ref subjects) = request.subjects {
            if subjects.is_empty() {
                return Err(ServiceError::ValidationError(
                    "Subjects list cannot be empty".to_string(),
                ));
            }
        }

        // Add more validations as needed
        Ok(())
    }
}

