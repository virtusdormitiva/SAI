use std::sync::Arc;
use uuid::Uuid;
use diesel::result::Error as DieselError;

use crate::{
    db::DbPool,
    models::{Course, CreateCourseDto, UpdateCourseDto},
    services::{ServiceError, ServiceResult},
};

/// Servicio para la gestión de cursos
pub struct CourseService {
    /// Pool de conexiones a la base de datos
    db_pool: Arc<DbPool>,
}

impl CourseService {
    /// Crea una nueva instancia del servicio de cursos
    ///
    /// # Arguments
    ///
    /// * `db_pool` - Pool de conexiones a la base de datos
    ///
    /// # Returns
    ///
    /// Una nueva instancia de CourseService
    pub fn new(db_pool: Arc<DbPool>) -> Self {
        Self { db_pool }
    }

    /// Obtiene todos los cursos con paginación
    ///
    /// # Arguments
    ///
    /// * `page` - Número de página
    /// * `page_size` - Tamaño de página
    ///
    /// # Returns
    ///
    /// Un vector con los cursos encontrados
    pub async fn get_all_courses(&self, page: u32, page_size: u32) -> ServiceResult<Vec<Course>> {
        let pool = self.db_pool.as_ref();
        Course::find_all(pool, page, page_size)
            .await
            .map_err(|e| ServiceError::DatabaseError(e.into()))
    }

    /// Obtiene un curso por su ID
    ///
    /// # Arguments
    ///
    /// * `id` - UUID del curso a buscar
    ///
    /// # Returns
    ///
    /// El curso encontrado o un error si no existe
    pub async fn get_course_by_id(&self, id: Uuid) -> ServiceResult<Course> {
        let pool = self.db_pool.as_ref();
        Course::find_by_id(pool, id)
            .await
            .map_err(|e| ServiceError::DatabaseError(e.into()))?
            .ok_or_else(|| ServiceError::NotFound(format!("Curso con ID {}", id)))
    }

    /// Obtiene un curso por su código
    ///
    /// # Arguments
    ///
    /// * `code` - Código del curso a buscar
    ///
    /// # Returns
    ///
    /// El curso encontrado o un error si no existe
    pub async fn get_course_by_code(&self, code: &str) -> ServiceResult<Course> {
        let pool = self.db_pool.as_ref();
        Course::find_by_code(pool, code)
            .await
            .map_err(|e| ServiceError::DatabaseError(e.into()))?
            .ok_or_else(|| ServiceError::NotFound(format!("Curso con código {}", code)))
    }

    /// Obtiene cursos por grado/nivel
    ///
    /// # Arguments
    ///
    /// * `grade_level` - Grado/nivel a buscar
    ///
    /// # Returns
    ///
    /// Un vector con los cursos del grado especificado
    pub async fn get_courses_by_grade_level(&self, grade_level: &str) -> ServiceResult<Vec<Course>> {
        let pool = self.db_pool.as_ref();
        Course::find_by_grade_level(pool, grade_level)
            .await
            .map_err(|e| ServiceError::DatabaseError(e.into()))
    }

    /// Obtiene cursos por profesor asignado
    ///
    /// # Arguments
    ///
    /// * `teacher_id` - ID del profesor
    ///
    /// # Returns
    ///
    /// Un vector con los cursos asignados al profesor
    pub async fn get_courses_by_teacher(&self, teacher_id: Uuid) -> ServiceResult<Vec<Course>> {
        let pool = self.db_pool.as_ref();
        Course::find_by_teacher(pool, teacher_id)
            .await
            .map_err(|e| ServiceError::DatabaseError(e.into()))
    }

    /// Obtiene cursos por año académico
    ///
    /// # Arguments
    ///
    /// * `academic_year` - Año académico a buscar
    ///
    /// # Returns
    ///
    /// Un vector con los cursos del año académico especificado
    pub async fn get_courses_by_academic_year(&self, academic_year: i32) -> ServiceResult<Vec<Course>> {
        let pool = self.db_pool.as_ref();
        Course::find_by_academic_year(pool, academic_year)
            .await
            .map_err(|e| ServiceError::DatabaseError(e.into()))
    }

    /// Obtiene cursos sin profesor asignado
    ///
    /// # Returns
    ///
    /// Un vector con los cursos sin profesor asignado
    pub async fn get_unassigned_courses(&self) -> ServiceResult<Vec<Course>> {
        let pool = self.db_pool.as_ref();
        Course::find_unassigned_courses(pool)
            .await
            .map_err(|e| ServiceError::DatabaseError(e.into()))
    }

    /// Busca cursos que coincidan con un término de búsqueda
    ///
    /// # Arguments
    ///
    /// * `term` - Término de búsqueda
    ///
    /// # Returns
    ///
    /// Un vector con los cursos que coinciden con el término de búsqueda
    pub async fn search_courses(&self, term: &str) -> ServiceResult<Vec<Course>> {
        let pool = self.db_pool.as_ref();
        Course::search(pool, term)
            .await
            .map_err(|e| ServiceError::DatabaseError(e.into()))
    }

    /// Crea un nuevo curso
    ///
    /// # Arguments
    ///
    /// * `dto` - Datos para crear el curso
    ///
    /// # Returns
    ///
    /// El curso creado
    pub async fn create_course(&self, dto: CreateCourseDto) -> ServiceResult<Course> {
        // Validar los datos del DTO
        self.validate_course_dto(&dto)?;
        
        // Verificar si ya existe un curso con el mismo código
        let pool = self.db_pool.as_ref();
        let existing = Course::find_by_code(pool, &dto.code)
            .await
            .map_err(|e| ServiceError::DatabaseError(e.into()))?;
            
        if existing.is_some() {
            return Err(ServiceError::ValidationError(
                format!("Ya existe un curso con el código {}", dto.code)
            ));
        }
        
        // Crear el curso
        Course::create(pool, dto)
            .await
            .map_err(|e| ServiceError::DatabaseError(e.into()))
    }

    /// Actualiza un curso existente
    ///
    /// # Arguments
    ///
    /// * `id` - UUID del curso a actualizar
    /// * `dto` - Datos para actualizar el curso
    ///
    /// # Returns
    ///
    /// El curso actualizado
    pub async fn update_course(&self, id: Uuid, dto: UpdateCourseDto) -> ServiceResult<Course> {
        // Obtener el curso existente
        let pool = self.db_pool.as_ref();
        let course = self.get_course_by_id(id).await?;
        
        // Validar el código si se está actualizando
        if let Some(ref code) = dto.code {
            if code != &course.code {
                let existing = Course::find_by_code(pool, code)
                    .await
                    .map_err(|e| ServiceError::DatabaseError(e.into()))?;
                    
                if existing.is_some() {
                    return Err(ServiceError::ValidationError(
                        format!("Ya existe un curso con el código {}", code)
                    ));
                }
            }
        }
        
        // Actualizar el curso
        course.update(pool, dto)
            .await
            .map_err(|e| ServiceError::DatabaseError(e.into()))
    }

    /// Elimina un curso
    ///
    /// # Arguments
    ///
    /// * `id` - UUID del curso a eliminar
    ///
    /// # Returns
    ///
    /// Ok(()) si la operación fue exitosa
    pub async fn delete_course(&self, id: Uuid) -> ServiceResult<()> {
        // Obtener el curso existente
        let pool = self.db_pool.as_ref();
        let course = self.get_course_by_id(id).await?;
        
        // Eliminar el curso
        course.delete(pool)
            .await
            .map_err(|e| ServiceError::DatabaseError(e.into()))
    }

    /// Asigna un profesor a un curso
    ///
    /// # Arguments
    ///
    /// * `course_id` - UUID del curso
    /// * `teacher_id` - UUID del profesor
    ///
    /// # Returns
    ///
    /// El curso actualizado con el nuevo profesor
    pub async fn assign_teacher(&self, course_id: Uuid, teacher_id: Uuid) -> ServiceResult<Course> {
        // Obtener el curso existente
        let pool = self.db_pool.as_ref();
        let course = self.get_course_by_id(course_id).await?;
        
        // Asignar el profesor
        course.assign_teacher(pool, teacher_id)
            .await
            .map_err(|e| ServiceError::DatabaseError(e.into()))
    }

    /// Elimina la asignación de profesor de un curso
    ///
    /// # Arguments
    ///
    /// * `course_id` - UUID del curso
    ///
    /// # Returns
    ///
    /// El curso actualizado sin profesor asignado
    pub async fn unassign_teacher(&self, course_id: Uuid) -> ServiceResult<Course> {
        // Obtener el curso existente
        let pool = self.db_pool.as_ref();
        let course = self.get_course_by_id(course_id).await?;
        
        // Quitar la asignación del profesor
        course.unassign_teacher(pool)
            .await
            .map_err(|e| ServiceError::DatabaseError(e.into()))
    }

    /// Obtiene estadísticas sobre los cursos por grado
    ///
    /// # Returns
    ///
    /// Un vector de tuplas con el grado y la cantidad de cursos
    pub async fn stats_by_grade(&self) -> ServiceResult<Vec<(String, i64)>> {
        let pool = self.db_pool.as_ref();
        Course::stats_by_grade(pool)
            .await
            .map_err(|e| ServiceError::DatabaseError(e.into()))
    }

    /// Obtiene estadísticas sobre los cursos por año académico
    ///
    /// # Returns
    ///
    /// Un vector de tuplas con el año académico y la cantidad de cursos
    pub async fn stats_by_academic_year(&self) -> ServiceResult<Vec<(i32, i64)>> {
        let pool = self.db_pool.as_ref();
        Course::stats_by_academic_year(pool)
            .await
            .map_err(|e| ServiceError::DatabaseError(e.into()))
    }

    /// Obtiene el número total de cursos
    ///
    /// # Returns
    ///
    /// La cantidad total de cursos
    pub async fn count_courses(&self) -> ServiceResult<i64> {
        let pool = self.db_pool.as_ref();
        Course::count(pool)
            .await
            .map_err(|e| ServiceError::DatabaseError(e.into()))
    }

    // Métodos privados auxiliares

    /// Valida los datos de un DTO de curso
    ///
    /// # Arguments
    ///
    /// * `dto` - DTO a validar
    ///
    /// # Returns
    ///
    /// Ok(()) si la validación es exitosa, o un error si falla
    fn validate_course_dto(&self, dto: &CreateCourseDto) -> ServiceResult<()> {
        // Validar código
        if dto.code.trim().is_empty() {
            return Err(ServiceError::ValidationError(
                "El código del curso no puede estar vacío".to_string()
            ));
        }
        
        // Validar nombre
        if dto.name.trim().is_empty() {
            return Err(ServiceError::ValidationError(
                "El nombre del curso no puede estar vacío".to_string()
            ));
        }
        
        // Validar grado
        if dto.grade_level.trim().is_empty() {
            return Err(ServiceError::ValidationError(
                "El grado del curso no puede estar vacío".to_string()
            ));
        }
        
        // Validar créditos
        if dto.credits <= 0.0 {
            return Err(ServiceError::ValidationError(
                "Los créditos del curso deben ser mayores a cero".to_string()
            ));
        }
        
        // Validar año académico
        if dto.academic_year < 2000 || dto.academic_year > 2100 {
            return Err(ServiceError::ValidationError(
                "El año académico debe estar entre 2000 y 2100".to_string()
            ));
        }
        
        Ok(())
    }
}

