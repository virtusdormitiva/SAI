use crate::models::{Course, ScheduleSlot, TeacherStatus};
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, postgres::PgPool, FromRow};
use uuid::Uuid;

/// Data Transfer Object para la creación de un nuevo curso
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCourseDto {
    /// Código del curso
    pub code: String,
    /// Nombre del curso
    pub name: String,
    /// Descripción detallada (opcional)
    pub description: Option<String>,
    /// Grado al que pertenece
    pub grade_level: String,
    /// Créditos académicos asignados
    pub credits: f32,
    /// Profesor asignado (opcional)
    pub teacher_id: Option<Uuid>,
    /// Año académico
    pub academic_year: i32,
    /// Horario semanal
    pub schedule: Vec<ScheduleSlot>,
}

/// Data Transfer Object para la actualización de un curso existente
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCourseDto {
    /// Código del curso (opcional)
    pub code: Option<String>,
    /// Nombre del curso (opcional)
    pub name: Option<String>,
    /// Descripción detallada (opcional)
    pub description: Option<String>,
    /// Grado al que pertenece (opcional)
    pub grade_level: Option<String>,
    /// Créditos académicos asignados (opcional)
    pub credits: Option<f32>,
    /// Profesor asignado (opcional)
    pub teacher_id: Option<Uuid>,
    /// Año académico (opcional)
    pub academic_year: Option<i32>,
    /// Horario semanal (opcional)
    pub schedule: Option<Vec<ScheduleSlot>>,
}

/// Implementación de métodos para el modelo de Curso
impl Course {
    /// Crea un nuevo curso en la base de datos
    pub async fn create(db: &Pool<Postgres>, dto: CreateCourseDto) -> Result<Self> {
        // Generar un nuevo UUID para el curso
        let id = Uuid::new_v4();
        
        // Convertir el horario a formato JSON para almacenamiento
        let schedule_json = serde_json::to_value(&dto.schedule)?;
        
        // Insertar el nuevo curso en la base de datos
        let course = sqlx::query_as!(
            Course,
            r#"
            INSERT INTO courses (
                id, code, name, description, grade_level, 
                credits, teacher_id, academic_year, schedule
            ) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) 
            RETURNING 
                id, code, name, description, grade_level, 
                credits, teacher_id, academic_year, 
                schedule as "schedule!: Vec<ScheduleSlot>"
            "#,
            id,
            dto.code,
            dto.name,
            dto.description,
            dto.grade_level,
            dto.credits,
            dto.teacher_id,
            dto.academic_year,
            schedule_json
        )
        .fetch_one(db)
        .await?;
        
        Ok(course)
    }
    
    /// Encuentra un curso por su ID
    pub async fn find_by_id(db: &Pool<Postgres>, id: Uuid) -> Result<Option<Self>> {
        let course = sqlx::query_as!(
            Course,
            r#"
            SELECT 
                id, code, name, description, grade_level, 
                credits, teacher_id, academic_year, 
                schedule as "schedule!: Vec<ScheduleSlot>"
            FROM courses 
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(db)
        .await?;
        
        Ok(course)
    }
    
    /// Encuentra un curso por su código
    pub async fn find_by_code(db: &Pool<Postgres>, code: &str) -> Result<Option<Self>> {
        let course = sqlx::query_as!(
            Course,
            r#"
            SELECT 
                id, code, name, description, grade_level, 
                credits, teacher_id, academic_year, 
                schedule as "schedule!: Vec<ScheduleSlot>"
            FROM courses 
            WHERE code = $1
            "#,
            code
        )
        .fetch_optional(db)
        .await?;
        
        Ok(course)
    }
    
    /// Encuentra cursos por grado/nivel
    pub async fn find_by_grade_level(db: &Pool<Postgres>, grade_level: &str) -> Result<Vec<Self>> {
        let courses = sqlx::query_as!(
            Course,
            r#"
            SELECT 
                id, code, name, description, grade_level, 
                credits, teacher_id, academic_year, 
                schedule as "schedule!: Vec<ScheduleSlot>"
            FROM courses 
            WHERE grade_level = $1
            ORDER BY name
            "#,
            grade_level
        )
        .fetch_all(db)
        .await?;
        
        Ok(courses)
    }
    
    /// Encuentra cursos por profesor asignado
    pub async fn find_by_teacher(db: &Pool<Postgres>, teacher_id: Uuid) -> Result<Vec<Self>> {
        let courses = sqlx::query_as!(
            Course,
            r#"
            SELECT 
                id, code, name, description, grade_level, 
                credits, teacher_id, academic_year, 
                schedule as "schedule!: Vec<ScheduleSlot>"
            FROM courses 
            WHERE teacher_id = $1
            ORDER BY name
            "#,
            teacher_id
        )
        .fetch_all(db)
        .await?;
        
        Ok(courses)
    }
    
    /// Encuentra cursos por año académico
    pub async fn find_by_academic_year(db: &Pool<Postgres>, academic_year: i32) -> Result<Vec<Self>> {
        let courses = sqlx::query_as!(
            Course,
            r#"
            SELECT 
                id, code, name, description, grade_level, 
                credits, teacher_id, academic_year, 
                schedule as "schedule!: Vec<ScheduleSlot>"
            FROM courses 
            WHERE academic_year = $1
            ORDER BY name
            "#,
            academic_year
        )
        .fetch_all(db)
        .await?;
        
        Ok(courses)
    }
    
    /// Encuentra cursos sin profesor asignado
    pub async fn find_unassigned_courses(db: &Pool<Postgres>) -> Result<Vec<Self>> {
        let courses = sqlx::query_as!(
            Course,
            r#"
            SELECT 
                id, code, name, description, grade_level, 
                credits, teacher_id, academic_year, 
                schedule as "schedule!: Vec<ScheduleSlot>"
            FROM courses 
            WHERE teacher_id IS NULL
            ORDER BY name
            "#
        )
        .fetch_all(db)
        .await?;
        
        Ok(courses)
    }
    
    /// Obtiene todos los cursos con paginación
    pub async fn find_all(
        db: &Pool<Postgres>, 
        page: u32, 
        page_size: u32
    ) -> Result<Vec<Self>> {
        let offset = (page - 1) * page_size;
        
        let courses = sqlx::query_as!(
            Course,
            r#"
            SELECT 
                id, code, name, description, grade_level, 
                credits, teacher_id, academic_year, 
                schedule as "schedule!: Vec<ScheduleSlot>"
            FROM courses 
            ORDER BY name
            LIMIT $1 OFFSET $2
            "#,
            page_size as i64,
            offset as i64
        )
        .fetch_all(db)
        .await?;
        
        Ok(courses)
    }
    
    /// Busca cursos que coincidan con un término de búsqueda
    pub async fn search(db: &Pool<Postgres>, term: &str) -> Result<Vec<Self>> {
        let search_term = format!("%{}%", term);
        
        let courses = sqlx::query_as!(
            Course,
            r#"
            SELECT 
                id, code, name, description, grade_level, 
                credits, teacher_id, academic_year, 
                schedule as "schedule!: Vec<ScheduleSlot>"
            FROM courses 
            WHERE 
                code ILIKE $1 OR 
                name ILIKE $1 OR 
                description ILIKE $1 OR
                grade_level ILIKE $1
            ORDER BY name
            "#,
            search_term
        )
        .fetch_all(db)
        .await?;
        
        Ok(courses)
    }
    
    /// Actualiza un curso existente
    pub async fn update(&self, db: &Pool<Postgres>, dto: UpdateCourseDto) -> Result<Self> {
        // Preparar los valores para actualizar
        let code = dto.code.unwrap_or_else(|| self.code.clone());
        let name = dto.name.unwrap_or_else(|| self.name.clone());
        let description = dto.description.or(self.description.clone());
        let grade_level = dto.grade_level.unwrap_or_else(|| self.grade_level.clone());
        let credits = dto.credits.unwrap_or(self.credits);
        let teacher_id = dto.teacher_id.or(self.teacher_id);
        let academic_year = dto.academic_year.unwrap_or(self.academic_year);
        
        let schedule = dto.schedule.unwrap_or_else(|| self.schedule.clone());
        let schedule_json = serde_json::to_value(&schedule)?;
        
        // Actualizar el curso en la base de datos
        let updated_course = sqlx::query_as!(
            Course,
            r#"
            UPDATE courses 
            SET 
                code = $1,
                name = $2,
                description = $3,
                grade_level = $4,
                credits = $5,
                teacher_id = $6,
                academic_year = $7,
                schedule = $8
            WHERE id = $9
            RETURNING 
                id, code, name, description, grade_level, 
                credits, teacher_id, academic_year, 
                schedule as "schedule!: Vec<ScheduleSlot>"
            "#,
            code,
            name,
            description,
            grade_level,
            credits,
            teacher_id,
            academic_year,
            schedule_json,
            self.id
        )
        .fetch_one(db)
        .await?;
        
        Ok(updated_course)
    }
    
    /// Elimina un curso de la base de datos
    pub async fn delete(&self, db: &Pool<Postgres>) -> Result<()> {
        sqlx::query!(
            "DELETE FROM courses WHERE id = $1",
            self.id
        )
        .execute(db)
        .await?;
        
        Ok(())
    }
    
    /// Asigna un profesor a un curso
    pub async fn assign_teacher(&self, db: &Pool<Postgres>, teacher_id: Uuid) -> Result<Self> {
        // Verificar que el profesor exista y esté activo
        let teacher_exists = sqlx::query!(
            r#"
            SELECT status as "status!: TeacherStatus" 
            FROM teachers 
            WHERE user_id = $1
            "#,
            teacher_id
        )
        .fetch_optional(db)
        .await?;
        
        if let Some(teacher) = teacher_exists {
            if teacher.status != TeacherStatus::Active {
                return Err(anyhow::anyhow!("El profesor no está activo"));
            }
        } else {
            return Err(anyhow::anyhow!("Profesor no encontrado"));
        }
        
        // Actualizar el curso con el nuevo profesor
        let updated_course = sqlx::query_as!(
            Course,
            r#"
            UPDATE courses 
            SET teacher_id = $1
            WHERE id = $2
            RETURNING 
                id, code, name, description, grade_level, 
                credits, teacher_id, academic_year, 
                schedule as "schedule!: Vec<ScheduleSlot>"
            "#,
            teacher_id,
            self.id
        )
        .fetch_one(db)
        .await?;
        
        Ok(updated_course)
    }
    
    /// Elimina la asignación de profesor de un curso
    pub async fn unassign_teacher(&self, db: &Pool<Postgres>) -> Result<Self> {
        // Actualizar el curso para quitar el profesor
        let updated_course = sqlx::query_as!(
            Course,
            r#"
            UPDATE courses 
            SET teacher_id = NULL
            WHERE id = $1
            RETURNING 
                id, code, name, description, grade_level, 
                credits, teacher_id, academic_year, 
                schedule as "schedule!: Vec<ScheduleSlot>"
            "#,
            self.id
        )
        .fetch_one(db)
        .await?;
        
        Ok(updated_course)
    }
    
    /// Obtiene el número total de cursos
    pub async fn count(db: &Pool<Postgres>) -> Result<i64> {
        let result = sqlx::query!(
            "SELECT COUNT(*) as count FROM courses"
        )
        .fetch_one(db)
        .await?;
        
        Ok(result.count.unwrap_or(0))
    }
    
    /// Obtiene estadísticas sobre los cursos por grado
    pub async fn stats_by_grade(db: &Pool<Postgres>) -> Result<Vec<(String, i64)>> {
        let rows = sqlx::query!(
            r#"
            SELECT grade_level, COUNT(*) as count
            FROM courses
            GROUP BY grade_level
            ORDER BY grade_level
            "#
        )
        .fetch_all(db)
        .await?;
        
        let stats = rows.into_iter()
            .map(|row| (row.grade_level, row.count.unwrap_or(0)))
            .collect();
        
        Ok(stats)
    }
    
    /// Obtiene estadísticas sobre los cursos por año académico
    pub async fn stats_by_academic_year(db: &Pool<Postgres>) -> Result<Vec<(i32, i64)>> {
        let rows = sqlx::query!(
            r#"
            SELECT academic_year, COUNT(*) as count
            FROM courses
            GROUP BY academic_year
            ORDER BY academic_year
            "#
        )
        .fetch_all(db)
        .await?;
        
        let stats = rows.into_iter()
            .map(|row| (row.academic_year, row.count.unwrap_or(0)))

            .collect();

        Ok(stats)
    }
}
