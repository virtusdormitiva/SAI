use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPool, Error as SqlxError, Postgres, Transaction};
use uuid::Uuid;

use crate::db::{DbError, DbPool, DEFAULT_PAGE_SIZE};

/// Represents the status of a student's attendance
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "attendance_status", rename_all = "lowercase")]
pub enum AttendanceStatus {
    Present,
    Absent,
    Late,
    Excused,
}

impl Default for AttendanceStatus {
    fn default() -> Self {
        Self::Present
    }
}

/// Represents a student's attendance record in the system
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Attendance {
    pub id: Uuid,
    pub student_id: Uuid,
    pub course_id: Uuid,
    pub date: NaiveDate,
    pub status: AttendanceStatus,
    pub notes: Option<String>,
    pub minutes_late: Option<i32>,
    pub recorded_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Input data for creating a new attendance record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAttendance {
    pub student_id: Uuid,
    pub course_id: Uuid,
    pub date: NaiveDate,
    pub status: AttendanceStatus,
    pub notes: Option<String>,
    pub minutes_late: Option<i32>,
    pub recorded_by: Uuid,
}

/// Input data for updating an existing attendance record
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AttendanceUpdate {
    pub status: Option<AttendanceStatus>,
    pub notes: Option<String>,
    pub minutes_late: Option<i32>,
    pub recorded_by: Option<Uuid>,
}

/// Filter parameters for attendance records
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AttendanceFilter {
    pub student_id: Option<Uuid>,
    pub course_id: Option<Uuid>,
    pub date_from: Option<NaiveDate>,
    pub date_to: Option<NaiveDate>,
    pub status: Option<AttendanceStatus>,
    pub recorded_by: Option<Uuid>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

/// Attendance statistics for a course or student
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttendanceStatistics {
    pub total_days: i64,
    pub present_days: i64,
    pub absent_days: i64,
    pub late_days: i64,
    pub excused_days: i64,
    pub attendance_rate: f64,
}

impl Attendance {
    /// Creates a new attendance record in the database
    pub async fn create(
        pool: &DbPool,
        new_attendance: NewAttendance,
    ) -> Result<Attendance, DbError> {
        let result = sqlx::query_as!(
            Attendance,
            r#"
            INSERT INTO attendances (
                student_id, course_id, date, status, notes, minutes_late, recorded_by, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW())
            RETURNING id, student_id, course_id, date, status as "status: AttendanceStatus", 
                      notes, minutes_late, recorded_by, created_at, updated_at
            "#,
            new_attendance.student_id,
            new_attendance.course_id,
            new_attendance.date,
            new_attendance.status as AttendanceStatus,
            new_attendance.notes,
            new_attendance.minutes_late,
            new_attendance.recorded_by
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    /// Creates a new attendance record in the database within a transaction
    pub async fn create_in_transaction(
        tx: &mut Transaction<'_, Postgres>,
        new_attendance: NewAttendance,
    ) -> Result<Attendance, DbError> {
        let result = sqlx::query_as!(
            Attendance,
            r#"
            INSERT INTO attendances (
                student_id, course_id, date, status, notes, minutes_late, recorded_by, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW())
            RETURNING id, student_id, course_id, date, status as "status: AttendanceStatus", 
                      notes, minutes_late, recorded_by, created_at, updated_at
            "#,
            new_attendance.student_id,
            new_attendance.course_id,
            new_attendance.date,
            new_attendance.status as AttendanceStatus,
            new_attendance.notes,
            new_attendance.minutes_late,
            new_attendance.recorded_by
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(result)
    }

    /// Retrieves an attendance record by ID
    pub async fn find_by_id(pool: &DbPool, id: Uuid) -> Result<Option<Attendance>, DbError> {
        let result = sqlx::query_as!(
            Attendance,
            r#"
            SELECT 
                id, student_id, course_id, date, status as "status: AttendanceStatus", 
                notes, minutes_late, recorded_by, created_at, updated_at
            FROM attendances
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }

    /// Retrieves attendance records by student and date
    pub async fn find_by_student_and_date(
        pool: &DbPool,
        student_id: Uuid,
        date: NaiveDate,
    ) -> Result<Vec<Attendance>, DbError> {
        let result = sqlx::query_as!(
            Attendance,
            r#"
            SELECT 
                id, student_id, course_id, date, status as "status: AttendanceStatus", 
                notes, minutes_late, recorded_by, created_at, updated_at
            FROM attendances
            WHERE student_id = $1 AND date = $2
            "#,
            student_id,
            date
        )
        .fetch_all(pool)
        .await?;

        Ok(result)
    }

    /// Filters attendance records based on provided criteria
    pub async fn filter(
        pool: &DbPool,
        filter: AttendanceFilter,
    ) -> Result<Vec<Attendance>, DbError> {
        let mut query = String::from(
            r#"
            SELECT 
                id, student_id, course_id, date, status as "status: AttendanceStatus", 
                notes, minutes_late, recorded_by, created_at, updated_at
            FROM attendances
            WHERE 1=1
            "#,
        );

        let mut params: Vec<Box<dyn sqlx::postgres::PgArguments>> = Vec::new();
        let mut param_count = 1;

        if let Some(student_id) = filter.student_id {
            query.push_str(&format!(" AND student_id = ${}", param_count));
            params.push(Box::new(student_id));
            param_count += 1;
        }

        if let Some(course_id) = filter.course_id {
            query.push_str(&format!(" AND course_id = ${}", param_count));
            params.push(Box::new(course_id));
            param_count += 1;
        }

        if let Some(date_from) = filter.date_from {
            query.push_str(&format!(" AND date >= ${}", param_count));
            params.push(Box::new(date_from));
            param_count += 1;
        }

        if let Some(date_to) = filter.date_to {
            query.push_str(&format!(" AND date <= ${}", param_count));
            params.push(Box::new(date_to));
            param_count += 1;
        }

        if let Some(status) = filter.status {
            query.push_str(&format!(" AND status = ${}", param_count));
            params.push(Box::new(status));
            param_count += 1;
        }

        if let Some(recorded_by) = filter.recorded_by {
            query.push_str(&format!(" AND recorded_by = ${}", param_count));
            params.push(Box::new(recorded_by));
            param_count += 1;
        }

        // Order by date (most recent first)
        query.push_str(" ORDER BY date DESC");

        // Add pagination
        let page = filter.page.unwrap_or(1);
        let page_size = filter.page_size.unwrap_or(DEFAULT_PAGE_SIZE);
        let offset = (page - 1) * page_size;
        query.push_str(&format!(" LIMIT ${} OFFSET ${}", param_count, param_count + 1));
        params.push(Box::new(page_size));
        params.push(Box::new(offset));

        // Unfortunately, we can't easily use the built query directly with sqlx::query_as! since it requires 
        // static strings. In a real implementation, we would use a more flexible query builder or the 
        // dynamic query capabilities of sqlx. For now, we'll simulate this with a simplified query.

        let result = sqlx::query_as!(
            Attendance,
            r#"
            SELECT 
                id, student_id, course_id, date, status as "status: AttendanceStatus", 
                notes, minutes_late, recorded_by, created_at, updated_at
            FROM attendances
            WHERE ($1::uuid IS NULL OR student_id = $1)
              AND ($2::uuid IS NULL OR course_id = $2)
              AND ($3::date IS NULL OR date >= $3)
              AND ($4::date IS NULL OR date <= $4)
              AND ($5::attendance_status IS NULL OR status = $5)
              AND ($6::uuid IS NULL OR recorded_by = $6)
            ORDER BY date DESC
            LIMIT $7 OFFSET $8
            "#,
            filter.student_id,
            filter.course_id,
            filter.date_from,
            filter.date_to,
            filter.status as Option<AttendanceStatus>,
            filter.recorded_by,
            page_size as i64,
            offset as i64
        )
        .fetch_all(pool)
        .await?;

        Ok(result)
    }

    /// Updates an attendance record
    pub async fn update(
        pool: &DbPool,
        id: Uuid,
        update: AttendanceUpdate,
    ) -> Result<Attendance, DbError> {
        // Check if the attendance record exists
        let attendance = Self::find_by_id(pool, id).await?;
        if attendance.is_none() {
            return Err(DbError::NotFound("Attendance record not found".to_string()));
        }

        let result = sqlx::query_as!(
            Attendance,
            r#"
            UPDATE attendances
            SET 
                status = COALESCE($1, status),
                notes = COALESCE($2, notes),
                minutes_late = COALESCE($3, minutes_late),
                recorded_by = COALESCE($4, recorded_by),
                updated_at = NOW()
            WHERE id = $5
            RETURNING id, student_id, course_id, date, status as "status: AttendanceStatus", 
                      notes, minutes_late, recorded_by, created_at, updated_at
            "#,
            update.status as Option<AttendanceStatus>,
            update.notes,
            update.minutes_late,
            update.recorded_by,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    /// Deletes an attendance record
    pub async fn delete(pool: &DbPool, id: Uuid) -> Result<(), DbError> {
        let result = sqlx::query!("DELETE FROM attendances WHERE id = $1", id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound("Attendance record not found".to_string()));
        }

        Ok(())
    }

    /// Bulk creates attendance records for multiple students in a course
    pub async fn bulk_create(
        pool: &DbPool,
        course_id: Uuid,
        student_ids: Vec<Uuid>,
        date: NaiveDate,
        status: AttendanceStatus,
        recorded_by: Uuid,
    ) -> Result<Vec<Attendance>, DbError> {
        let mut tx = pool.begin().await?;
        let mut created_records = Vec::new();

        for student_id in student_ids {
            let new_attendance = NewAttendance {
                student_id,
                course_id,
                date,
                status: status.clone(),
                notes: None,
                minutes_late: None,
                recorded_by,
            };

            let attendance = Self::create_in_transaction(&mut tx, new_attendance).await?;
            created_records.push(attendance);
        }

        tx.commit().await?;
        Ok(created_records)
    }

    /// Gets attendance statistics for a student in a course
    pub async fn get_student_statistics(
        pool: &DbPool,
        student_id: Uuid,
        course_id: Uuid,
    ) -> Result<AttendanceStatistics, DbError> {
        let result = sqlx::query!(
            r#"
            SELECT 
                COUNT(*) as "total_days!",
                COUNT(*) FILTER (WHERE status = 'present') as "present_days!",
                COUNT(*) FILTER (WHERE status = 'absent') as "absent_days!",
                COUNT(*) FILTER (WHERE status = 'late') as "late_days!",
                COUNT(*) FILTER (WHERE status = 'excused') as "excused_days!"
            FROM attendances
            WHERE student_id = $1 AND course_id = $2
            "#,
            student_id,
            course_id
        )
        .fetch_one(pool)
        .await?;

        let attendance_rate = if result.total_days > 0 {
            (result.present_days as f64 + result.excused_days as f64) / result.total_days as f64
        } else {
            0.0
        };

        Ok(AttendanceStatistics {
            total_days: result.total_days,
            present_days: result.present_days,
            absent_days: result.absent_days,
            late_days: result.late_days,
            excused_days: result.excused_days,
            attendance_rate,
        })
    }

    /// Validates a new attendance record
    pub fn validate_new_attendance(
        new_attendance: &NewAtten

