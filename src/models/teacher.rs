use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Error as SqlxError, postgres::PgQueryResult};
use uuid::Uuid;

use crate::models::{TeacherStatus, User};

/// Re-exportamos Teacher para facilitar su uso en el módulo models
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Teacher {
    /// Referencia al usuario base
    pub user_id: Uuid,
    /// Número de registro profesional
    pub professional_id: String,
    /// Especialidad del profesor
    pub specialization: String,
    /// Fecha de contratación
    pub hire_date: NaiveDate,
    /// Nivel de educación (licenciatura, maestría, etc.)
    pub education_level: String,
    /// Materias que puede enseñar
    pub subjects: Vec<String>,
    /// Estado laboral (activo, licencia, etc.)
    pub status: TeacherStatus,
    /// Fecha de creación del registro
    pub created_at: DateTime<Utc>,
    /// Última actualización del registro
    pub updated_at: DateTime<Utc>,
}

/// DTO para la creación de un nuevo profesor
#[derive(Debug, Deserialize)]
pub struct CreateTeacherDto {
    pub user_id: Uuid,
    pub professional_id: String,
    pub specialization: String,
    pub hire_date: NaiveDate,
    pub education_level: String,
    pub subjects: Vec<String>,
    pub status: TeacherStatus,
}

/// DTO para la actualización de un profesor
#[derive(Debug, Deserialize)]
pub struct UpdateTeacherDto {
    pub professional_id: Option<String>,
    pub specialization: Option<String>,
    pub hire_date: Option<NaiveDate>,
    pub education_level: Option<String>,
    pub subjects: Option<Vec<String>>,
    pub status: Option<TeacherStatus>,
}

/// Filtros para la búsqueda de profesores
#[derive(Debug, Deserialize, Default)]
pub struct TeacherFilter {
    pub user_id: Option<Uuid>,
    pub professional_id: Option<String>,
    pub specialization: Option<String>,
    pub status: Option<TeacherStatus>,
    pub subject: Option<String>,
}

/// DTO para devolver la información completa de un profesor (datos de usuario + datos de profesor)
#[derive(Debug, Serialize)]
pub struct TeacherWithUserData {
    // Campos del usuario
    pub id: Uuid,
    pub document_id: String,
    pub full_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub birth_date: NaiveDate,
    // Campos del profesor
    pub professional_id: String,
    pub specialization: String,
    pub hire_date: NaiveDate,
    pub education_level: String,
    pub subjects: Vec<String>,
    pub status: TeacherStatus,
}

impl Teacher {
    /// Crea un nuevo profesor en la base de datos
    pub async fn create(pool: &PgPool, dto: CreateTeacherDto) -> Result<Teacher, SqlxError> {
        let now = Utc::now();

        // Verificar que el usuario existe
        let user_exists = User::find_by_id(pool, dto.user_id).await?;
        if user_exists.is_none() {
            return Err(SqlxError::RowNotFound);
        }

        // Convertir Vec<String> a formato JSON para almacenar en PostgreSQL
        let subjects_json = serde_json::to_value(&dto.subjects).unwrap();

        let teacher = sqlx::query_as!(
            Teacher,
            r#"
            INSERT INTO teachers (
                user_id, professional_id, specialization, hire_date, 
                education_level, subjects, status, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING 
                user_id, professional_id, specialization, hire_date, 
                education_level, subjects as "subjects: Vec<String>", 
                status as "status: TeacherStatus", created_at, updated_at
            "#,
            dto.user_id,
            dto.professional_id,
            dto.specialization,
            dto.hire_date,
            dto.education_level,
            subjects_json,
            dto.status as TeacherStatus,
            now,
            now
        )
        .fetch_one(pool)
        .await?;

        Ok(teacher)
    }

    /// Encuentra un profesor por ID de usuario
    pub async fn find_by_user_id(pool: &PgPool, user_id: Uuid) -> Result<Option<Teacher>, SqlxError> {
        let teacher = sqlx::query_as!(
            Teacher,
            r#"
            SELECT 
                user_id, professional_id, specialization, hire_date, 
                education_level, subjects as "subjects: Vec<String>", 
                status as "status: TeacherStatus", created_at, updated_at
            FROM teachers
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(teacher)
    }

    /// Encuentra un profesor por su número de registro profesional
    pub async fn find_by_professional_id(pool: &PgPool, professional_id: &str) -> Result<Option<Teacher>, SqlxError> {
        let teacher = sqlx::query_as!(
            Teacher,
            r#"
            SELECT 
                user_id, professional_id, specialization, hire_date, 
                education_level, subjects as "subjects: Vec<String>", 
                status as "status: TeacherStatus", created_at, updated_at
            FROM teachers
            WHERE professional_id = $1
            "#,
            professional_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(teacher)
    }

    /// Lista todos los profesores con opción de filtrado y paginación
    pub async fn find_all(
        pool: &PgPool, 
        filter: TeacherFilter,
        limit: Option<i64>,
        offset: Option<i64>
    ) -> Result<Vec<Teacher>, SqlxError> {
        // Construimos la consulta base
        let mut query = String::from(
            "SELECT user_id, professional_id, specialization, hire_date, 
            education_level, subjects, status, created_at, updated_at 
            FROM teachers WHERE 1=1"
        );

        // Aplicamos los filtros si existen
        let mut params = Vec::<String>::new();
        let mut param_count = 1;

        if let Some(user_id) = filter.user_id {
            query.push_str(&format!(" AND user_id = ${}", param_count));
            params.push(user_id.to_string());
            param_count += 1;
        }

        if let Some(professional_id) = &filter.professional_id {
            query.push_str(&format!(" AND professional_id = ${}", param_count));
            params.push(professional_id.to_string());
            param_count += 1;
        }

        if let Some(specialization) = &filter.specialization {
            query.push_str(&format!(" AND specialization ILIKE ${}", param_count));
            params.push(format!("%{}%", specialization));
            param_count += 1;
        }

        if let Some(status) = &filter.status {
            query.push_str(&format!(" AND status = ${}", param_count));
            params.push(format!("{:?}", status));
            param_count += 1;
        }

        if let Some(subject) = &filter.subject {
            // Buscar en el array de subjects
            query.push_str(&format!(" AND subjects @> ${}::jsonb", param_count));
            params.push(format!("[\"{}\"]\", subject));
            param_count += 1;
        }

        // Agregamos paginación
        query.push_str(" ORDER BY created_at DESC");

        if let Some(limit_val) = limit {
            query.push_str(&format!(" LIMIT ${}", param_count));
            params.push(limit_val.to_string());
            param_count += 1;
        }

        if let Some(offset_val) = offset {
            query.push_str(&format!(" OFFSET ${}", param_count));
            params.push(offset_val.to_string());
        }

        // Ejecutamos la consulta dinámica
        let mut q = sqlx::query(&query);
        for param in params {
            q = q.bind(param);
        }

        // Convertimos el resultado a instancias de Teacher
        let rows = q.fetch_all(pool).await?;
        let teachers = rows
            .iter()
            .map(|row| {
                Teacher {
                    user_id: row.get("user_id"),
                    professional_id: row.get("professional_id"),
                    specialization: row.get("specialization"),
                    hire_date: row.get("hire_date"),
                    education_level: row.get("education_level"),
                    subjects: serde_json::from_value(row.get("subjects")).unwrap_or_default(),
                    status: serde_json::from_value(row.get("status")).unwrap_or(TeacherStatus::Active),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                }
            })
            .collect();

        Ok(teachers)
    }

    /// Actualiza un profesor existente
    pub async fn update(pool: &PgPool, user_id: Uuid, dto: UpdateTeacherDto) -> Result<Teacher, SqlxError> {
        // Primero verificamos si el profesor existe
        let existing_teacher = Self::find_by_user_id(pool, user_id).await?;
        if existing_teacher.is_none() {
            return Err(SqlxError::RowNotFound);
        }

        let existing_teacher = existing_teacher.unwrap();
        let now = Utc::now();

        // Usamos los valores actuales si no se especifican nuevos
        let professional_id = dto.professional_id.unwrap_or(existing_teacher.professional_id);
        let specialization = dto.specialization.unwrap_or(existing_teacher.specialization);
        let hire_date = dto.hire_date.unwrap_or(existing_teacher.hire_date);
        let education_level = dto.education_level.unwrap_or(existing_teacher.education_level);
        let subjects = dto.subjects.unwrap_or(existing_teacher.subjects);
        let status = dto.status.unwrap_or(existing_teacher.status);

        // Convertir Vec<String> a formato JSON para almacenar en PostgreSQL
        let subjects_json = serde_json::to_value(&subjects).unwrap();

        let updated_teacher = sqlx::query_as!(
            Teacher,
            r#"
            UPDATE teachers 
            SET professional_id = $1, specialization = $2, hire_date = $3, 
                education_level = $4, subjects = $5, status = $6, updated_at = $7
            WHERE user_id = $8
            RETURNING 
                user_id, professional_id, specialization, hire_date, 
                education_level, subjects as "subjects: Vec<String>", 
                status as "status: TeacherStatus", created_at, updated_at
            "#,
            professional_id,
            specialization,
            hire_date,
            education_level,
            subjects_json,
            status as TeacherStatus,
            now,
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(updated_teacher)
    }

    /// Elimina un profesor por su ID de usuario
    pub async fn delete(pool: &PgPool, user_id: Uuid) -> Result<PgQueryResult, SqlxError> {
        // Verificamos si el profesor existe
        let existing_teacher = Self::find_by_user_id(pool, user_id).await?;
        if existing_teacher.is_none() {
            return Err(SqlxError::RowNotFound);
        }

        let result = sqlx::query!(
            r#"
            DELETE FROM teachers
            WHERE user_id = $1
            "#,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(result)
    }

    /// Obtiene la información completa de un profesor (datos de usuario + datos de profesor)
    pub async fn get_teacher_with_user_data(pool: &PgPool, user_id: Uuid) -> Result<Option<TeacherWithUserData>, SqlxError> {
        let teacher_with_user = sqlx::query!(
            r#"
            SELECT 
                u.id, u.document_id, u.full_name, u.email, u.phone, u.address, u.birth_date,
                t.professional_id, t.specialization, t.hire_date, t.education_level, 
                t.subjects as "subjects: Vec<String>", t.status as "status: TeacherStatus"
            FROM teachers t
            JOIN users u ON t.user_id = u.id
            WHERE t.user_id = $1
            "#,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        match teacher_with_user {
            Some(record) => Ok(Some(TeacherWithUserData {
                id: record.id,
                document_id: record.document_id,
                full_name: record.full_name,
                email: record.email,
                phone: record.phone,
                address: record.address,
                birth_date: record.birth_date,
                professional_id: record.professional_id,
                specialization: record.specialization,
                hire_date: record.hire_date,
                education_level: record.education_level,
                subjects: record.subjects,
                status: record.status,
            })),
            None => Ok(None),
        }
    }

    /// Cuenta el número total de profesores que coinciden con un filtro
    pub async fn count(pool: &PgPool, filter: TeacherFilter) -> Result<i64, SqlxError> {
        // Construimos la consulta base
        let mut query = String::from("SELECT COUNT(*) FROM teachers WHERE 1=1");

        // Aplicamos los filtros si existen
        let mut params = Vec::<String>::new();
        let mut param_count = 1;

        if let Some(user_id) = filter.user_id {
            query.push_str(&format!(" AND user_id = ${}", param_count));
            params.push

