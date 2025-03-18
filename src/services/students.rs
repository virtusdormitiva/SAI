use actix_web::{http::StatusCode, web, HttpResponse};
use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{
    student::{CreateStudentDto, CreateStudentWithUserDto, Student, StudentFilter, UpdateStudentDto},
    GuardianInfo, StudentStatus,
};
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateStudentRequest {
    pub user_id: Uuid,
    pub enrollment_number: String,
    pub current_grade: String,
    pub section: String,
    pub academic_year: i32,
    pub guardian_info: Option<GuardianInfo>,
    pub status: StudentStatus,
};
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateStudentRequest {
    pub enrollment_number: Option<String>,
    pub current_grade: Option<String>,
    pub section: Option<String>,
    pub academic_year: Option<i32>,
    pub guardian_info: Option<GuardianInfo>,
    pub status: Option<StudentStatus>,
};

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Student not found")]
    NotFound,
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Internal server error: {0}")]
    InternalServerError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
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


pub struct StudentService {
    pool: web::Data<PgPool>,
}

impl StudentService {
    pub fn new(pool: web::Data<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn get_all_students(&self, filter: Option<StudentFilter>, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<Student>, ServiceError> {
        let filter = filter.unwrap_or_default();
        
        Student::find_all(&self.pool, filter, limit, offset)
            .await
            .map_err(|e| ServiceError::InternalServerError(e.to_string()))
    }
    pub async fn get_student_by_id(&self, user_id: Uuid) -> Result<Student, ServiceError> {
        Student::find_by_user_id(&self.pool, user_id)
            .await
            .map_err(|e| ServiceError::InternalServerError(e.to_string()))
            .and_then(|maybe_student| {
                maybe_student.ok_or(ServiceError::NotFound)
            })
    }
    
    pub async fn get_student_by_enrollment_number(&self, enrollment_number: &str) -> Result<Student, ServiceError> {
        Student::find_by_enrollment_number(&self.pool, enrollment_number)
            .await
            .map_err(|e| ServiceError::InternalServerError(e.to_string()))
            .and_then(|maybe_student| {
                maybe_student.ok_or(ServiceError::NotFound)
            })
    }
    pub async fn create_student(
        &self,
        request: CreateStudentRequest,
    ) -> Result<Student, ServiceError> {
        // Validate the request
        Self::validate_create_student(&request)?;

        let dto = CreateStudentDto {
            user_id: request.user_id,
            enrollment_number: request.enrollment_number,
            current_grade: request.current_grade,
            section: request.section,
            academic_year: request.academic_year,
            guardian_info: request.guardian_info,
            status: request.status,
        };

        Student::create(&self.pool, dto)
            .await
            .map_err(|e| ServiceError::InternalServerError(e.to_string()))
    }
    
    pub async fn create_student_with_user(
        &self,
        request: CreateStudentWithUserDto,
    ) -> Result<(crate::models::User, Student), ServiceError> {
        Student::create_with_user(&self.pool, request)
            .await
            .map_err(|e| ServiceError::InternalServerError(e.to_string()))
    }
    pub async fn update_student(
        &self,
        user_id: Uuid,
        request: UpdateStudentRequest,
    ) -> Result<Student, ServiceError> {
        // First, check if the student exists
        self.get_student_by_id(user_id).await?;

        // Validate the request
        Self::validate_update_student(&request)?;

        let dto = UpdateStudentDto {
            enrollment_number: request.enrollment_number,
            current_grade: request.current_grade,
            section: request.section,
            academic_year: request.academic_year,
            guardian_info: request.guardian_info,
            status: request.status,
        };

        Student::update(&self.pool, user_id, dto)
            .await
            .map_err(|e| ServiceError::InternalServerError(e.to_string()))
    }
    pub async fn delete_student(&self, user_id: Uuid) -> Result<(), ServiceError> {
        // First, check if the student exists
        self.get_student_by_id(user_id).await?;

        Student::delete(&self.pool, user_id)
            .await
            .map_err(|e| ServiceError::InternalServerError(e.to_string()))
            .map(|_| ())
    }

    // Helper methods for validation
    fn validate_create_student(request: &CreateStudentRequest) -> Result<(), ServiceError> {
        if request.enrollment_number.is_empty() {
            return Err(ServiceError::ValidationError(
                "Enrollment number cannot be empty".to_string(),
            ));
        }
        
        if request.current_grade.is_empty() {
            return Err(ServiceError::ValidationError(
                "Current grade cannot be empty".to_string(),
            ));
        }
        
        if request.section.is_empty() {
            return Err(ServiceError::ValidationError(
                "Section cannot be empty".to_string(),
            ));
        }

        // Add more validations as needed
        Ok(())
    }

    fn validate_update_student(request: &UpdateStudentRequest) -> Result<(), ServiceError> {
        if let Some(ref enrollment_number) = request.enrollment_number {
            if enrollment_number.is_empty() {
                return Err(ServiceError::ValidationError(
                    "Enrollment number cannot be empty".to_string(),
                ));
            }
        }
        
        if let Some(ref current_grade) = request.current_grade {
            if current_grade.is_empty() {
                return Err(ServiceError::ValidationError(
                    "Current grade cannot be empty".to_string(),
                ));
            }
        }
        
        if let Some(ref section) = request.section {
            if section.is_empty() {
                return Err(ServiceError::ValidationError(
                    "Section cannot be empty".to_string(),
                ));
            }
        }

        // Add more validations as needed
        Ok(())
    }

