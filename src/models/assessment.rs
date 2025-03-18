use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, Transaction};
use uuid::Uuid;

/// Represents the type of assessment
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AssessmentType {
    Quiz,
    Test,
    Assignment,
    Project,
    Exam,
    Participation,
    Other(String),
}

/// Represents an assessment record in the database
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Assessment {
    pub id: Uuid,
    pub enrollment_id: Uuid,
    pub course_id: Uuid,
    pub assessment_type: AssessmentType,
    pub title: String,
    pub description: Option<String>,
    pub score: f64,
    pub max_score: f64,
    pub weight: f64,
    pub assessment_date: DateTime<Utc>,
    pub is_final: bool,
    pub comments: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Represents the data needed to create a new assessment
#[derive(Debug, Serialize, Deserialize)]
pub struct NewAssessment {
    pub enrollment_id: Uuid,
    pub course_id: Uuid,
    pub assessment_type: AssessmentType,
    pub title: String,
    pub description: Option<String>,
    pub score: f64,
    pub max_score: f64,
    pub weight: f64,
    pub assessment_date: DateTime<Utc>,
    pub is_final: bool,
    pub comments: Option<String>,
}

/// Represents the data needed to update an existing assessment
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssessmentUpdate {
    pub assessment_type: Option<AssessmentType>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub score: Option<f64>,
    pub max_score: Option<f64>,
    pub weight: Option<f64>,
    pub assessment_date: Option<DateTime<Utc>>,
    pub is_final: Option<bool>,
    pub comments: Option<String>,
}

/// Filter options for querying assessments
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssessmentFilter {
    pub enrollment_id: Option<Uuid>,
    pub course_id: Option<Uuid>,
    pub assessment_type: Option<AssessmentType>,
    pub title: Option<String>,
    pub is_final: Option<bool>,
    pub min_score: Option<f64>,
    pub max_score: Option<f64>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

impl Assessment {
    /// Create a new assessment in the database
    pub async fn create(
        pool: &Pool<Postgres>,
        new_assessment: NewAssessment,
    ) -> Result<Self, sqlx::Error> {
        // Validate the new assessment data
        Self::validate_new_assessment(&new_assessment)?;

        let assessment = sqlx::query_as!(
            Assessment,
            r#"
            INSERT INTO assessments (
                enrollment_id, course_id, assessment_type, title, description,
                score, max_score, weight, assessment_date, is_final, comments
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING
                id, enrollment_id, course_id, assessment_type as "assessment_type: AssessmentType",
                title, description, score, max_score, weight, assessment_date,
                is_final, comments, created_at, updated_at
            "#,
            new_assessment.enrollment_id,
            new_assessment.course_id,
            new_assessment.assessment_type as _,
            new_assessment.title,
            new_assessment.description,
            new_assessment.score,
            new_assessment.max_score,
            new_assessment.weight,
            new_assessment.assessment_date,
            new_assessment.is_final,
            new_assessment.comments
        )
        .fetch_one(pool)
        .await?;

        Ok(assessment)
    }

    /// Get an assessment by its ID
    pub async fn get_by_id(pool: &Pool<Postgres>, id: Uuid) -> Result<Self, sqlx::Error> {
        let assessment = sqlx::query_as!(
            Assessment,
            r#"
            SELECT
                id, enrollment_id, course_id, assessment_type as "assessment_type: AssessmentType",
                title, description, score, max_score, weight, assessment_date,
                is_final, comments, created_at, updated_at
            FROM assessments
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;

        Ok(assessment)
    }

    /// Get assessments by filter
    pub async fn get_by_filter(
        pool: &Pool<Postgres>,
        filter: AssessmentFilter,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let mut query = "
            SELECT
                id, enrollment_id, course_id, assessment_type as \"assessment_type: AssessmentType\",
                title, description, score, max_score, weight, assessment_date,
                is_final, comments, created_at, updated_at
            FROM assessments
            WHERE 1 = 1"
            .to_string();

        let mut params = Vec::new();
        let mut param_index = 1;

        if let Some(enrollment_id) = filter.enrollment_id {
            query.push_str(&format!(" AND enrollment_id = ${}", param_index));
            params.push(enrollment_id.to_string());
            param_index += 1;
        }

        if let Some(course_id) = filter.course_id {
            query.push_str(&format!(" AND course_id = ${}", param_index));
            params.push(course_id.to_string());
            param_index += 1;
        }

        if let Some(assessment_type) = filter.assessment_type {
            query.push_str(&format!(" AND assessment_type = ${}", param_index));
            params.push(format!("{:?}", assessment_type).to_lowercase());
            param_index += 1;
        }

        if let Some(title) = filter.title {
            query.push_str(&format!(" AND title ILIKE ${}", param_index));
            params.push(format!("%{}%", title));
            param_index += 1;
        }

        if let Some(is_final) = filter.is_final {
            query.push_str(&format!(" AND is_final = ${}", param_index));
            params.push(is_final.to_string());
            param_index += 1;
        }

        if let Some(min_score) = filter.min_score {
            query.push_str(&format!(" AND score >= ${}", param_index));
            params.push(min_score.to_string());
            param_index += 1;
        }

        if let Some(max_score) = filter.max_score {
            query.push_str(&format!(" AND score <= ${}", param_index));
            params.push(max_score.to_string());
            param_index += 1;
        }

        if let Some(start_date) = filter.start_date {
            query.push_str(&format!(" AND assessment_date >= ${}", param_index));
            params.push(start_date.to_rfc3339());
            param_index += 1;
        }

        if let Some(end_date) = filter.end_date {
            query.push_str(&format!(" AND assessment_date <= ${}", param_index));
            params.push(end_date.to_rfc3339());
            param_index += 1;
        }

        query.push_str(" ORDER BY assessment_date DESC");

        let assessments = sqlx::query_as(&query)
            .fetch_all(pool)
            .await?;

        Ok(assessments)
    }

    /// Update an assessment by its ID
    pub async fn update(
        pool: &Pool<Postgres>,
        id: Uuid,
        update: AssessmentUpdate,
    ) -> Result<Self, sqlx::Error> {
        // Validate the update data
        Self::validate_update(&update)?;

        let current = Self::get_by_id(pool, id).await?;
        
        let assessment = sqlx::query_as!(
            Assessment,
            r#"
            UPDATE assessments
            SET
                assessment_type = COALESCE($1, assessment_type),
                title = COALESCE($2, title),
                description = COALESCE($3, description),
                score = COALESCE($4, score),
                max_score = COALESCE($5, max_score),
                weight = COALESCE($6, weight),
                assessment_date = COALESCE($7, assessment_date),
                is_final = COALESCE($8, is_final),
                comments = COALESCE($9, comments),
                updated_at = NOW()
            WHERE id = $10
            RETURNING
                id, enrollment_id, course_id, assessment_type as "assessment_type: AssessmentType",
                title, description, score, max_score, weight, assessment_date,
                is_final, comments, created_at, updated_at
            "#,
            update.assessment_type as _,
            update.title,
            update.description,
            update.score,
            update.max_score,
            update.weight,
            update.assessment_date,
            update.is_final,
            update.comments,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(assessment)
    }

    /// Delete an assessment by its ID
    pub async fn delete(pool: &Pool<Postgres>, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM assessments WHERE id = $1", id)
            .execute(pool)
            .await?;

        Ok(())
    }

    /// Create multiple assessments in a transaction
    pub async fn create_batch(
        tx: &mut Transaction<'_, Postgres>,
        assessments: Vec<NewAssessment>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let mut created_assessments = Vec::new();

        for assessment in assessments {
            // Validate each assessment
            Self::validate_new_assessment(&assessment)?;

            let created = sqlx::query_as!(
                Assessment,
                r#"
                INSERT INTO assessments (
                    enrollment_id, course_id, assessment_type, title, description,
                    score, max_score, weight, assessment_date, is_final, comments
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                RETURNING
                    id, enrollment_id, course_id, assessment_type as "assessment_type: AssessmentType",
                    title, description, score, max_score, weight, assessment_date,
                    is_final, comments, created_at, updated_at
                "#,
                assessment.enrollment_id,
                assessment.course_id,
                assessment.assessment_type as _,
                assessment.title,
                assessment.description,
                assessment.score,
                assessment.max_score,
                assessment.weight,
                assessment.assessment_date,
                assessment.is_final,
                assessment.comments
            )
            .fetch_one(&mut **tx)
            .await?;

            created_assessments.push(created);
        }

        Ok(created_assessments)
    }

    /// Calculate the weighted average of all assessments for a student in a course
    pub async fn calculate_weighted_average(
        pool: &Pool<Postgres>,
        enrollment_id: Uuid,
        course_id: Uuid,
    ) -> Result<f64, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT 
                SUM(score * weight) / SUM(weight) as weighted_average
            FROM assessments
            WHERE enrollment_id = $1 AND course_id = $2
            "#,
            enrollment_id,
            course_id
        )
        .fetch_one(pool)
        .await?;

        Ok(result.weighted_average.unwrap_or(0.0))
    }

    /// Calculate the overall grade based on weighted average and grading scale
    pub async fn calculate_grade(
        pool: &Pool<Postgres>,
        enrollment_id: Uuid,
        course_id: Uuid,
    ) -> Result<String, sqlx::Error> {
        let weighted_avg = Self::calculate_weighted_average(pool, enrollment_id, course_id).await?;
        
        // Apply grading scale
        let grade = if weighted_avg >= 90.0 {
            "A"
        } else if weighted_avg >= 80.0 {
            "B"
        } else if weighted_avg >= 70.0 {
            "C"
        } else if weighted_avg >= 60.0 {
            "D"
        } else {
            "F"
        };
        
        Ok(grade.to_string())
    }

    /// Calculate statistics for assessments in a course
    pub async fn calculate_course_statistics(
        pool: &Pool<Postgres>,
        course_id: Uuid,
    ) -> Result<CourseStatistics, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT 
                AVG(score / max_score * 100) as average_score,
                MIN(score / max_score * 100) as min_score,
                MAX(score / max_score * 100) as max_score,
                PERCENTILE_CONT(0.5) WITHIN GROUP (ORDER BY score / max_score * 100) as median_score,
                COUNT(*) as assessment_count
            FROM assessments
            WHERE course_id = $1
            "#,
            course_id
        )
        .fetch_one(pool)
        .await?;

        Ok(CourseStatistics {
            average_score: result.average_score.unwrap_or(0.0),
            min_score: result.min_score.unwrap_or(0.0),
            max_score: result.max_score.unwrap_or(0.0),
            median_score: result.median_score.unwrap_or(0.0),
            assessment_count: result.assessment_count.unwrap_or(0) as i32,
        })
    }

    /// Calculate grade distribution for a course
    pub async fn calculate_grade_distribution(
        pool: &Pool<Postgres>,
        course_id: Uuid,
    ) -> Result<GradeDistribution, sqlx::Error> {
        // Get all enrollments for this course
        let enrollments = sqlx::query!(
            "SELECT id FROM enrollments WHERE course_id = $1",
            course_id
        )
        .fetch_all(pool)
        .await?;

        let mut a_count = 0;
        let mut b_count = 0;
        let mut c_count = 0;
        let mut d_count = 0;
        let mut f_count = 0;

        // Calculate grade for each enrollment
        for enrollment in enrollments {
            let grade = Self::calculate_grade(pool, enrollment.id, course_i

