use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Error, Pool, Postgres, Row};
use uuid::Uuid;

use crate::db::DbPool;
use crate::models::{Course, Student};

/// Status of a student's enrollment in a course
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnrollmentStatus {
    /// Student is actively enrolled in the course
    Active,
    /// Student has withdrawn from the course
    Withdrawn,
    /// Student has completed the course
    Completed,
    /// Student is on hold/pause in the course
    OnHold,
    /// Student is pending approval for enrollment
    Pending,
}

impl std::fmt::Display for EnrollmentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnrollmentStatus::Active => write!(f, "active"),
            EnrollmentStatus::Withdrawn => write!(f, "withdrawn"),
            EnrollmentStatus::Completed => write!(f, "completed"),
            EnrollmentStatus::OnHold => write!(f, "on_hold"),
            EnrollmentStatus::Pending => write!(f, "pending"),
        }
    }
}

impl From<&str> for EnrollmentStatus {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "active" => EnrollmentStatus::Active,
            "withdrawn" => EnrollmentStatus::Withdrawn,
            "completed" => EnrollmentStatus::Completed,
            "on_hold" => EnrollmentStatus::OnHold,
            "pending" => EnrollmentStatus::Pending,
            _ => EnrollmentStatus::Active, // Default to active if unknown
        }
    }
}

/// Represents a student's enrollment in a course
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enrollment {
    /// Unique identifier for the enrollment
    pub id: Uuid,
    /// Reference to the student who is enrolled
    pub student_id: Uuid,
    /// Reference to the course the student is enrolled in
    pub course_id: Uuid,
    /// Date when the student enrolled in the course
    pub enrollment_date: DateTime<Utc>,
    /// Current status of the enrollment (active, withdrawn, completed, etc.)
    pub status: EnrollmentStatus,
    /// Optional completion date if the student has completed the course
    pub completion_date: Option<DateTime<Utc>>,
    /// Optional final grade for the course
    pub final_grade: Option<f64>,
    /// Optional notes or comments about the enrollment
    pub notes: Option<String>,
    /// Optional payment information (could be a JSON string with payment details)
    pub payment_info: Option<serde_json::Value>,
    /// When the enrollment record was created
    pub created_at: DateTime<Utc>,
    /// When the enrollment record was last updated
    pub updated_at: DateTime<Utc>,
}

/// Data required to create a new enrollment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewEnrollment {
    /// ID of the student to enroll
    pub student_id: Uuid,
    /// ID of the course to enroll in
    pub course_id: Uuid,
    /// Initial status of the enrollment (defaults to Pending if not provided)
    pub status: Option<EnrollmentStatus>,
    /// Optional notes or comments about the enrollment
    pub notes: Option<String>,
    /// Optional payment information
    pub payment_info: Option<serde_json::Value>,
}

/// Data for updating an existing enrollment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrollmentUpdate {
    /// Updated status of the enrollment
    pub status: Option<EnrollmentStatus>,
    /// Updated completion date
    pub completion_date: Option<DateTime<Utc>>,
    /// Updated final grade
    pub final_grade: Option<f64>,
    /// Updated notes or comments
    pub notes: Option<String>,
    /// Updated payment information
    pub payment_info: Option<serde_json::Value>,
}

impl Enrollment {
    /// Create a new enrollment in the database
    pub async fn create(db: &DbPool, new_enrollment: &NewEnrollment) -> Result<Self, Error> {
        // Validate student and course existence
        Self::validate_student_course(db, new_enrollment.student_id, new_enrollment.course_id).await?;
        
        // Check if student is already enrolled in this course
        Self::check_existing_enrollment(db, new_enrollment.student_id, new_enrollment.course_id).await?;
        
        let status = new_enrollment.status.unwrap_or(EnrollmentStatus::Pending);
        
        let enrollment = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO enrollments (student_id, course_id, status, notes, payment_info)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, student_id, course_id, enrollment_date, 
                      status as "status: EnrollmentStatus", completion_date, final_grade, 
                      notes, payment_info, created_at, updated_at
            "#,
            new_enrollment.student_id,
            new_enrollment.course_id,
            status.to_string(),
            new_enrollment.notes,
            new_enrollment.payment_info
        )
        .fetch_one(db)
        .await?;
        
        Ok(enrollment)
    }
    
    /// Validate that both student and course exist
    async fn validate_student_course(db: &DbPool, student_id: Uuid, course_id: Uuid) -> Result<(), Error> {
        // Check if student exists
        let student_exists = sqlx::query!("SELECT id FROM students WHERE id = $1", student_id)
            .fetch_optional(db)
            .await?
            .is_some();
        
        if !student_exists {
            return Err(Error::RowNotFound);
        }
        
        // Check if course exists
        let course_exists = sqlx::query!("SELECT id FROM courses WHERE id = $1", course_id)
            .fetch_optional(db)
            .await?
            .is_some();
        
        if !course_exists {
            return Err(Error::RowNotFound);
        }
        
        Ok(())
    }
    
    /// Check if student is already enrolled in this course
    async fn check_existing_enrollment(db: &DbPool, student_id: Uuid, course_id: Uuid) -> Result<(), Error> {
        let existing = sqlx::query!(
            "SELECT id FROM enrollments WHERE student_id = $1 AND course_id = $2 AND status != 'withdrawn'",
            student_id,
            course_id
        )
        .fetch_optional(db)
        .await?;
        
        if existing.is_some() {
            return Err(Error::RowNotFound); // Using RowNotFound as a placeholder for a custom error
        }
        
        Ok(())
    }
    
    /// Retrieve an enrollment by its ID
    pub async fn find_by_id(db: &DbPool, id: Uuid) -> Result<Self, Error> {
        let enrollment = sqlx::query_as!(
            Self,
            r#"
            SELECT id, student_id, course_id, enrollment_date, 
                   status as "status: EnrollmentStatus", completion_date, final_grade,
                   notes, payment_info, created_at, updated_at
            FROM enrollments
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(db)
        .await?;
        
        Ok(enrollment)
    }
    
    /// Retrieve all enrollments for a specific student
    pub async fn find_by_student(db: &DbPool, student_id: Uuid) -> Result<Vec<Self>, Error> {
        let enrollments = sqlx::query_as!(
            Self,
            r#"
            SELECT id, student_id, course_id, enrollment_date, 
                   status as "status: EnrollmentStatus", completion_date, final_grade,
                   notes, payment_info, created_at, updated_at
            FROM enrollments
            WHERE student_id = $1
            "#,
            student_id
        )
        .fetch_all(db)
        .await?;
        
        Ok(enrollments)
    }
    
    /// Retrieve all enrollments for a specific course
    pub async fn find_by_course(db: &DbPool, course_id: Uuid) -> Result<Vec<Self>, Error> {
        let enrollments = sqlx::query_as!(
            Self,
            r#"
            SELECT id, student_id, course_id, enrollment_date, 
                   status as "status: EnrollmentStatus", completion_date, final_grade,
                   notes, payment_info, created_at, updated_at
            FROM enrollments
            WHERE course_id = $1
            "#,
            course_id
        )
        .fetch_all(db)
        .await?;
        
        Ok(enrollments)
    }
    
    /// Retrieve all enrollments with a specific status
    pub async fn find_by_status(db: &DbPool, status: EnrollmentStatus) -> Result<Vec<Self>, Error> {
        let enrollments = sqlx::query_as!(
            Self,
            r#"
            SELECT id, student_id, course_id, enrollment_date, 
                   status as "status: EnrollmentStatus", completion_date, final_grade,
                   notes, payment_info, created_at, updated_at
            FROM enrollments
            WHERE status = $1
            "#,
            status.to_string()
        )
        .fetch_all(db)
        .await?;
        
        Ok(enrollments)
    }
    
    /// Update an enrollment with new data
    pub async fn update(db: &DbPool, id: Uuid, update: &EnrollmentUpdate) -> Result<Self, Error> {
        let mut query = String::from("UPDATE enrollments SET updated_at = NOW()");
        let mut params: Vec<String> = Vec::new();
        let mut param_values: Vec<Box<dyn sqlx::Encode<'_, sqlx::Postgres> + Send>> = Vec::new();
        
        let mut param_index = 1;
        
        // Conditionally add each field to the update query
        if let Some(status) = &update.status {
            query.push_str(&format!(", status = ${}", param_index));
            params.push(status.to_string());
            param_values.push(Box::new(status.to_string()));
            param_index += 1;
        }
        
        if let Some(completion_date) = &update.completion_date {
            query.push_str(&format!(", completion_date = ${}", param_index));
            params.push(completion_date.to_string());
            param_values.push(Box::new(completion_date.clone()));
            param_index += 1;
        }
        
        if let Some(final_grade) = &update.final_grade {
            query.push_str(&format!(", final_grade = ${}", param_index));
            params.push(final_grade.to_string());
            param_values.push(Box::new(*final_grade));
            param_index += 1;
        }
        
        if let Some(notes) = &update.notes {
            query.push_str(&format!(", notes = ${}", param_index));
            params.push(notes.to_string());
            param_values.push(Box::new(notes.clone()));
            param_index += 1;
        }
        
        if let Some(payment_info) = &update.payment_info {
            query.push_str(&format!(", payment_info = ${}", param_index));
            params.push(payment_info.to_string());
            param_values.push(Box::new(payment_info.clone()));
            param_index += 1;
        }
        
        // Add the WHERE clause and RETURNING statement
        query.push_str(&format!(" WHERE id = ${} RETURNING id, student_id, course_id, enrollment_date, status as \"status: EnrollmentStatus\", completion_date, final_grade, notes, payment_info, created_at, updated_at", param_index));
        params.push(id.to_string());
        param_values.push(Box::new(id));
        
        // If there are no fields to update, just return the current enrollment
        if param_index == 1 {
            return Self::find_by_id(db, id).await;
        }
        
        // Execute the query
        let enrollment = sqlx::query_as::<_, Self>(&query)
            .fetch_one(db)
            .await?;
        
        Ok(enrollment)
    }
    
    /// Delete an enrollment from the database
    pub async fn delete(db: &DbPool, id: Uuid) -> Result<(), Error> {
        sqlx::query!("DELETE FROM enrollments WHERE id = $1", id)
            .execute(db)
            .await?;
        
        Ok(())
    }
    
    /// Withdraw a student from a course (special case of update)
    pub async fn withdraw(db: &DbPool, id: Uuid, notes: Option<String>) -> Result<Self, Error> {
        let update = EnrollmentUpdate {
            status: Some(EnrollmentStatus::Withdrawn),
            completion_date: None,
            final_grade: None,
            notes,
            payment_info: None,
        };
        
        Self::update(db, id, &update).await
    }
    
    /// Complete a student's enrollment with a final grade
    pub async fn complete(db: &DbPool, id: Uuid, final_grade: Option<f64>) -> Result<Self, Error> {
        let update = EnrollmentUpdate {
            status: Some(EnrollmentStatus::Completed),
            completion_date: Some(Utc::now()),
            final_grade,
            notes: None,
            payment_info: None,
        };
        
        Self::update(db, id, &update).await
    }
    
    /// Get enrollment with student and course details
    pub async fn get_with_details(db: &DbPool, id: Uuid) -> Result<EnrollmentDetails, Error> {
        let enrollment = Self::find_by_id(db, id).await?;
        let student = Student::find_by_id(db, enrollment.student_id).await?;
        let course = Course::find_by_id(db, enrollment.course_id).await?;
        
        Ok(EnrollmentDetails {
            enrollment,
            student,
            course,
        })
    }
}

/// Contains enrollment details with related student and course information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrollmentDetails {
    /// The enrollment record
    pub enrollment: Enrollment,
    /// The student record
    pub student: Student,
    /// The course record
    pub course: Course,
}

