//! Módulo de servicios de negocio
//!
//! Este módulo contiene la implementación de los servicios que manejan
//! la lógica de negocio de la aplicación. Cada servicio se encarga de una
//! entidad o funcionalidad específica del sistema.

use crate::models;
use std::sync::Arc;

// Módulos para cada tipo de servicio
pub mod users;
pub mod students;
pub mod teachers;
pub mod courses;
pub mod attendance;
pub mod grades;
pub mod schedules;
pub mod reports;
pub mod notifications;
pub mod payments;

// Re-exportación de servicios para uso fácil
pub use users::UserService;
pub use students::StudentService;
pub use teachers::TeacherService;
pub use courses::CourseService;
pub use attendance::AttendanceService;
pub use grades::GradeService;
pub use schedules::ScheduleService;
pub use reports::ReportService;
pub use notifications::NotificationService;
pub use payments::PaymentService;

/// Estructura que contiene todos los servicios de la aplicación
pub struct Services {
    /// Servicio para gestión de usuarios
    pub users: Arc<UserService>,
    /// Servicio para gestión de estudiantes
    pub students: Arc<StudentService>,
    /// Servicio para gestión de profesores
    pub teachers: Arc<TeacherService>,
    /// Servicio para gestión de cursos
    pub courses: Arc<CourseService>,
    /// Servicio para gestión de asistencia
    pub attendance: Arc<AttendanceService>,
    /// Servicio para gestión de calificaciones
    pub grades: Arc<GradeService>,
    /// Servicio para gestión de horarios
    pub schedules: Arc<ScheduleService>,
    /// Servicio para generación de reportes
    pub reports: Arc<ReportService>,
    /// Servicio para envío de notificaciones
    pub notifications: Arc<NotificationService>,
    /// Servicio para gestión de pagos
    pub payments: Arc<PaymentService>,
}

impl Services {
    /// Crea una nueva instancia de Services con todos los servicios inicializados
    ///
    /// # Arguments
    ///
    /// * `db_pool` - Pool de conexiones a la base de datos
    ///
    /// # Returns
    ///
    /// Una nueva instancia de Services
    pub fn new(db_pool: Arc<crate::db::DbPool>) -> Self {
        Self {
            users: Arc::new(UserService::new(db_pool.clone())),
            students: Arc::new(StudentService::new(db_pool.clone())),
            teachers: Arc::new(TeacherService::new(db_pool.clone())),
            courses: Arc::new(CourseService::new(db_pool.clone())),
            attendance: Arc::new(AttendanceService::new(db_pool.clone())),
            grades: Arc::new(GradeService::new(db_pool.clone())),
            schedules: Arc::new(ScheduleService::new(db_pool.clone())),
            reports: Arc::new(ReportService::new(db_pool.clone())),
            notifications: Arc::new(NotificationService::new(db_pool.clone())),
            payments: Arc::new(PaymentService::new(db_pool.clone())),
        }
    }
}

/// Error genérico para los servicios de negocio
#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    /// Error de base de datos
    #[error("Error de base de datos: {0}")]
    DatabaseError(#[from] diesel::result::Error),
    
    /// Entidad no encontrada
    #[error("{0} no encontrado/a")]
    NotFound(String),
    
    /// Error de validación
    #[error("Error de validación: {0}")]
    ValidationError(String),
    
    /// Error de autenticación
    #[error("Error de autenticación: {0}")]
    AuthenticationError(String),
    
    /// Error de autorización
    #[error("Error de autorización: {0}")]
    AuthorizationError(String),
    
    /// Error genérico
    #[error("{0}")]
    GenericError(String),
}

/// Resultado de operaciones de servicio
pub type ServiceResult<T> = Result<T, ServiceError>;

