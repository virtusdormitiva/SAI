//! Módulo de modelos para el Sistema Administrativo Integral (SAI)
//!
//! Este módulo contiene todas las estructuras de datos que representan
//! las entidades principales del sistema administrativo escolar.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Submódulos
pub mod user;
pub mod student;
pub mod teacher;
pub mod course;
pub mod enrollment;
pub mod attendance;
pub mod grade;
pub mod payment;
pub mod institution;

// Re-exportaciones para facilitar el acceso
pub use user::User;
pub use student::Student;
pub use teacher::Teacher;
pub use course::Course;
pub use enrollment::Enrollment;
pub use attendance::Attendance;
pub use grade::Grade;
pub use payment::Payment;
pub use institution::Institution;

/// Enumeración que representa los diferentes roles de usuario en el sistema
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Role {
    Admin,
    Director,
    Teacher,
    Student,
    Parent,
    Secretary,
    Accountant,
}

/// Estructura básica para el Usuario que sirve como base para estudiantes y profesores
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Identificador único del usuario
    pub id: Uuid,
    /// Número de documento de identidad (cédula)
    pub document_id: String,
    /// Nombre completo del usuario
    pub full_name: String,
    /// Correo electrónico de contacto
    pub email: String,
    /// Número de teléfono de contacto
    pub phone: Option<String>,
    /// Dirección física del usuario
    pub address: Option<String>,
    /// Fecha de nacimiento
    pub birth_date: chrono::NaiveDate,
    /// Rol del usuario en el sistema
    pub role: Role,
    /// Fecha de creación del registro
    pub created_at: DateTime<Utc>,
    /// Última actualización del registro
    pub updated_at: DateTime<Utc>,
}

/// Estructura que representa a un Estudiante en el sistema
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Información del tutor o encargado del estudiante
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardianInfo {
    /// Nombre completo del tutor
    pub name: String,
    /// Relación con el estudiante (padre, madre, etc.)
    pub relationship: String,
    /// Número de documento de identidad
    pub document_id: String,
    /// Correo electrónico de contacto
    pub email: Option<String>,
    /// Número de teléfono de contacto
    pub phone: String,
}

/// Estado posible de un estudiante
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StudentStatus {
    Active,
    Suspended,
    Withdrawn,
    Graduated,
    Transferred,
}

/// Estructura que representa a un Profesor en el sistema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Teacher {
    /// Referencia al usuario base
    pub user_id: Uuid,
    /// Número de registro profesional
    pub professional_id: String,
    /// Especialidad del profesor
    pub specialization: String,
    /// Fecha de contratación
    pub hire_date: chrono::NaiveDate,
    /// Nivel de educación (licenciatura, maestría, etc.)
    pub education_level: String,
    /// Materias que puede enseñar
    pub subjects: Vec<String>,
    /// Estado laboral (activo, licencia, etc.)
    pub status: TeacherStatus,
}

/// Estado posible de un profesor
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TeacherStatus {
    Active,
    OnLeave,
    Retired,
    Suspended,
    Terminated,
}

/// Estructura que representa un Curso o Materia en el sistema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    /// Identificador único del curso
    pub id: Uuid,
    /// Código del curso
    pub code: String,
    /// Nombre del curso
    pub name: String,
    /// Descripción detallada
    pub description: Option<String>,
    /// Grado al que pertenece
    pub grade_level: String,
    /// Créditos académicos asignados
    pub credits: f32,
    /// Profesor asignado
    pub teacher_id: Option<Uuid>,
    /// Año académico
    pub academic_year: i32,
    /// Horario semanal
    pub schedule: Vec<ScheduleSlot>,
}

/// Estructura que representa un espacio en el horario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleSlot {
    /// Día de la semana (1-7, donde 1 es lunes)
    pub day_of_week: u8,
    /// Hora de inicio
    pub start_time: String,
    /// Hora de finalización
    pub end_time: String,
    /// Aula o salón
    pub classroom: String,
}

/// Estructura que representa la inscripción de un estudiante a un curso
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enrollment {
    /// Identificador único
    pub id: Uuid,
    /// Estudiante inscrito
    pub student_id: Uuid,
    /// Curso al que se inscribe
    pub course_id: Uuid,
    /// Fecha de inscripción
    pub enrollment_date: DateTime<Utc>,
    /// Estado de la inscripción
    pub status: EnrollmentStatus,
    /// Notas o comentarios
    pub notes: Option<String>,
}

/// Estado de una inscripción
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EnrollmentStatus {
    Active,
    Withdrawn,
    Completed,
    Failed,
}

/// Institución educativa
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Institution {
    /// Identificador único
    pub id: Uuid,
    /// Nombre de la institución
    pub name: String,
    /// RUC o identificador fiscal
    pub tax_id: String,
    /// Dirección física
    pub address: String,
    /// Teléfono de contacto
    pub phone: String,
    /// Correo electrónico
    pub email: String,
    /// Sitio web
    pub website: Option<String>,
    /// Director o responsable
    pub director_name: String,
    /// Logo de la institución (ruta al archivo)
    pub logo_path: Option<String>,
    /// Año de fundación
    pub foundation_year: i32,
    /// Niveles educativos ofrecidos
    pub education_levels: Vec<String>,
}

/// Estructura para almacenar pagos y transacciones financieras
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    /// Identificador único
    pub id: Uuid,
    /// Estudiante relacionado
    pub student_id: Uuid,
    /// Concepto del pago (matrícula, mensualidad, etc.)
    pub concept: String,
    /// Monto del pago
    pub amount: f64,
    /// Moneda (Gs., USD, etc.)
    pub currency: String,
    /// Fecha del pago
    pub payment_date: DateTime<Utc>,
    /// Método de pago (efectivo, transferencia, etc.)
    pub payment_method: String,
    /// Estado del pago
    pub status: PaymentStatus,
    /// Número de comprobante o factura
    pub receipt_number: Option<String>,
    /// Notas adicionales
    pub notes: Option<String>,
}

/// Estado de un pago
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PaymentStatus {
    Pending,
    Completed,
    Cancelled,
    Refunded,
    Overdue,
}

/// Estructura para almacenar calificaciones
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grade {
    /// Identificador único
    pub id: Uuid,
    /// Estudiante evaluado
    pub student_id: Uuid,
    /// Curso evaluado
    pub course_id: Uuid,
    /// Tipo de evaluación (examen, trabajo práctico, etc.)
    pub evaluation_type: String,
    /// Valor numérico de la calificación
    pub value: f32,
    /// Escala (1-5, 1-10, etc.)
    pub scale: u8,
    /// Fecha de la evaluación
    pub evaluation_date: chrono::NaiveDate,
    /// Profesor que asignó la calificación
    pub teacher_id: Uuid,
    /// Comentarios adicionales
    pub comments: Option<String>,
}

/// Estructura para registro de asistencia
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attendance {
    /// Identificador único
    pub id: Uuid,
    /// Estudiante
    pub student_id: Uuid,
    /// Curso al que asistió
    pub course_id: Uuid,
    /// Fecha de asistencia
    pub date: chrono::NaiveDate,
    /// Estado de asistencia
    pub status: AttendanceStatus,
    /// Justificación en caso de ausencia
    pub justification: Option<String>,
    /// Registrado por (profesor o administrativo)
    pub recorded_by: Uuid,
}

/// Estado de asistencia
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AttendanceStatus {
    Present,
    Absent,
    Late,
    JustifiedAbsence,
}

