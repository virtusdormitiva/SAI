use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Error as SqlxError, postgres::PgQueryResult};
use uuid::Uuid;

use crate::models::{GuardianInfo, StudentStatus, Role, User};

/// Re-exportamos Student para facilitar su uso en el módulo models
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Student {
    /// Referencia al usuario base
    pub user_id: Uuid,
    /// Número de matrícula del estudiante
    pub enrollment_number: String,
    /// Grado o curso actual
    pub current_grade: String,
    /// Sección o división del grado
    pub section: String,
    /// Año académico actual
    pub academic_year: i32,
    /// Información del padre/madre/tutor
    pub guardian_info: Option<GuardianInfo>,
    /// Estado académico (activo, suspendido, etc.)
    pub status: StudentStatus,
}

/// DTO para la creación de un nuevo estudiante
#[derive(Debug, Deserialize)]
pub struct CreateStudentDto {
    pub user_id: Uuid,
    pub enrollment_number: String,
    pub current_grade: String,
    pub section: String,
    pub academic_year: i32,
    pub guardian_info: Option<GuardianInfo>,
    pub status: StudentStatus,
}

/// DTO para la actualización de un estudiante
#[derive(Debug, Deserialize)]
pub struct UpdateStudentDto {
    pub enrollment_number: Option<String>,
    pub current_grade: Option<String>,
    pub section: Option<String>,
    pub academic_year: Option<i32>,
    pub guardian_info: Option<GuardianInfo>,
    pub status: Option<StudentStatus>,
}

/// DTO para crear un estudiante junto con sus datos de usuario
#[derive(Debug, Deserialize)]
pub struct CreateStudentWithUserDto {
    // Datos del usuario
    pub document_id: String,
    pub full_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub birth_date: NaiveDate,
    
    // Datos específicos del estudiante
    pub enrollment_number: String,
    pub current_grade: String,
    pub section: String,
    pub academic_year: i32,
    pub guardian_info: Option<GuardianInfo>,
    pub status: StudentStatus,
}

/// Filtros para la búsqueda de estudiantes
#[derive(Debug, Deserialize, Default)]
pub struct StudentFilter {
    pub user_id: Option<Uuid>,
    pub enrollment_number: Option<String>,
    pub current_grade: Option<String>,
    pub section: Option<String>,
    pub academic_year: Option<i32>,
    pub status: Option<StudentStatus>,
    pub guardian_name: Option<String>,
}

impl Student {
    /// Crea un nuevo estudiante en la base de datos
    pub async fn create(pool: &PgPool, dto: CreateStudentDto) -> Result<Student, SqlxError> {
        // Verificar que el usuario exista antes de crear el estudiante
        let user_exists = User::find_by_id(pool, dto.user_id).await?;
        if user_exists.is_none() {
            return Err(SqlxError::RowNotFound);
        }

        let student = sqlx::query_as!(
            Student,
            r#"
            INSERT INTO students (
                user_id, enrollment_number, current_grade, section, 
                academic_year, guardian_info, status
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING 
                user_id, enrollment_number, current_grade, section, 
                academic_year, guardian_info as "guardian_info: Option<GuardianInfo>", 
                status as "status: StudentStatus"
            "#,
            dto.user_id,
            dto.enrollment_number,
            dto.current_grade,
            dto.section,
            dto.academic_year,
            serde_json::to_value(&dto.guardian_info)?,
            dto.status as StudentStatus
        )
        .fetch_one(pool)
        .await?;

        Ok(student)
    }

    /// Crea un nuevo estudiante junto con su usuario en una transacción
    pub async fn create_with_user(
        pool: &PgPool, 
        dto: CreateStudentWithUserDto
    ) -> Result<(User, Student), SqlxError> {
        // Iniciar transacción para garantizar atomicidad
        let mut tx = pool.begin().await?;

        // Crear el usuario primero
        let user_dto = crate::models::user::CreateUserDto {
            document_id: dto.document_id,
            full_name: dto.full_name,
            email: dto.email,
            phone: dto.phone,
            address: dto.address,
            birth_date: dto.birth_date,
            role: Role::Student, // Asignamos automáticamente el rol de estudiante
        };

        let user = User::create(&mut tx, user_dto).await?;

        // Crear el estudiante usando el ID del usuario recién creado
        let student_dto = CreateStudentDto {
            user_id: user.id,
            enrollment_number: dto.enrollment_number,
            current_grade: dto.current_grade,
            section: dto.section,
            academic_year: dto.academic_year,
            guardian_info: dto.guardian_info,
            status: dto.status,
        };

        let student = sqlx::query_as!(
            Student,
            r#"
            INSERT INTO students (
                user_id, enrollment_number, current_grade, section, 
                academic_year, guardian_info, status
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING 
                user_id, enrollment_number, current_grade, section, 
                academic_year, guardian_info as "guardian_info: Option<GuardianInfo>", 
                status as "status: StudentStatus"
            "#,
            student_dto.user_id,
            student_dto.enrollment_number,
            student_dto.current_grade,
            student_dto.section,
            student_dto.academic_year,
            serde_json::to_value(&student_dto.guardian_info)?,
            student_dto.status as StudentStatus
        )
        .fetch_one(&mut tx)
        .await?;

        // Confirmar la transacción
        tx.commit().await?;

        Ok((user, student))
    }

    /// Encuentra un estudiante por el ID de usuario
    pub async fn find_by_user_id(pool: &PgPool, user_id: Uuid) -> Result<Option<Student>, SqlxError> {
        let student = sqlx::query_as!(
            Student,
            r#"
            SELECT 
                user_id, enrollment_number, current_grade, section, 
                academic_year, guardian_info as "guardian_info: Option<GuardianInfo>", 
                status as "status: StudentStatus"
            FROM students
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(student)
    }

    /// Encuentra un estudiante por su número de matrícula
    pub async fn find_by_enrollment_number(pool: &PgPool, enrollment_number: &str) -> Result<Option<Student>, SqlxError> {
        let student = sqlx::query_as!(
            Student,
            r#"
            SELECT 
                user_id, enrollment_number, current_grade, section, 
                academic_year, guardian_info as "guardian_info: Option<GuardianInfo>", 
                status as "status: StudentStatus"
            FROM students
            WHERE enrollment_number = $1
            "#,
            enrollment_number
        )
        .fetch_optional(pool)
        .await?;

        Ok(student)
    }

    /// Lista todos los estudiantes con opción de filtrado y paginación
    pub async fn find_all(
        pool: &PgPool, 
        filter: StudentFilter,
        limit: Option<i64>,
        offset: Option<i64>
    ) -> Result<Vec<Student>, SqlxError> {
        // Construimos la consulta base
        let mut query = String::from(
            "SELECT user_id, enrollment_number, current_grade, section, 
                    academic_year, guardian_info, status 
             FROM students WHERE 1=1"
        );

        // Aplicamos los filtros si existen
        let mut params = Vec::<String>::new();
        let mut param_count = 1;

        if let Some(user_id) = filter.user_id {
            query.push_str(&format!(" AND user_id = ${}", param_count));
            params.push(user_id.to_string());
            param_count += 1;
        }

        if let Some(enrollment_number) = &filter.enrollment_number {
            query.push_str(&format!(" AND enrollment_number = ${}", param_count));
            params.push(enrollment_number.to_string());
            param_count += 1;
        }

        if let Some(current_grade) = &filter.current_grade {
            query.push_str(&format!(" AND current_grade = ${}", param_count));
            params.push(current_grade.to_string());
            param_count += 1;
        }

        if let Some(section) = &filter.section {
            query.push_str(&format!(" AND section = ${}", param_count));
            params.push(section.to_string());
            param_count += 1;
        }

        if let Some(academic_year) = filter.academic_year {
            query.push_str(&format!(" AND academic_year = ${}", param_count));
            params.push(academic_year.to_string());
            param_count += 1;
        }

        if let Some(status) = &filter.status {
            query.push_str(&format!(" AND status = ${}", param_count));
            params.push(format!("{:?}", status));
            param_count += 1;
        }

        if let Some(guardian_name) = &filter.guardian_name {
            query.push_str(&format!(" AND guardian_info->>'name' ILIKE ${}", param_count));
            params.push(format!("%{}%", guardian_name));
            param_count += 1;
        }

        // Agregamos ordenamiento y paginación
        query.push_str(" ORDER BY current_grade, section, enrollment_number");

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

        // Convertimos el resultado a instancias de Student
        let rows = q.fetch_all(pool).await?;
        let students = rows
            .iter()
            .map(|row| {
                Student {
                    user_id: row.get("user_id"),
                    enrollment_number: row.get("enrollment_number"),
                    current_grade: row.get("current_grade"),
                    section: row.get("section"),
                    academic_year: row.get("academic_year"),
                    guardian_info: serde_json::from_value(row.get("guardian_info")).unwrap_or(None),
                    status: serde_json::from_value(row.get("status")).unwrap_or(StudentStatus::Active),
                }
            })
            .collect();

        Ok(students)
    }

    /// Actualiza un estudiante existente
    pub async fn update(pool: &PgPool, user_id: Uuid, dto: UpdateStudentDto) -> Result<Student, SqlxError> {
        // Primero verificamos si el estudiante existe
        let existing_student = Self::find_by_user_id(pool, user_id).await?;
        if existing_student.is_none() {
            return Err(SqlxError::RowNotFound);
        }

        let existing_student = existing_student.unwrap();

        // Usamos los valores actuales si no se especifican nuevos
        let enrollment_number = dto.enrollment_number.unwrap_or(existing_student.enrollment_number);
        let current_grade = dto.current_grade.unwrap_or(existing_student.current_grade);
        let section = dto.section.unwrap_or(existing_student.section);
        let academic_year = dto.academic_year.unwrap_or(existing_student.academic_year);
        let guardian_info = dto.guardian_info.or(existing_student.guardian_info);
        let status = dto.status.unwrap_or(existing_student.status);

        let updated_student = sqlx::query_as!(
            Student,
            r#"
            UPDATE students 
            SET enrollment_number = $1, current_grade = $2, section = $3, 
                academic_year = $4, guardian_info = $5, status = $6
            WHERE user_id = $7
            RETURNING 
                user_id, enrollment_number, current_grade, section, 
                academic_year, guardian_info as "guardian_info: Option<GuardianInfo>", 
                status as "status: StudentStatus"
            "#,
            enrollment_number,
            current_grade,
            section,
            academic_year,
            serde_json::to_value(&guardian_info)?,
            status as StudentStatus,
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(updated_student)
    }

    /// Elimina un estudiante por su ID de usuario
    pub async fn delete(pool: &PgPool, user_id: Uuid) -> Result<PgQueryResult, SqlxError> {
        // Verificamos si el estudiante existe
        let existing_student = Self::find_by_user_id(pool, user_id).await?;
        if existing_student.is_none() {
            return Err(SqlxError::RowNotFound);
        }

        let result = sqlx::query!(
            r#"
            DELETE FROM students
            WHERE user_id = $1
            "#

