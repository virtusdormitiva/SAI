//! # Sistema Administrativo Integral (SAI)
//!
//! `sai` es una biblioteca diseñada para facilitar la gestión administrativa
//! integral de instituciones educativas y otras organizaciones en Paraguay.
//! Implementa funcionalidades que cumplen con normativas locales y estándares
//! de seguridad para el manejo de datos.

/// Módulo que contiene las estructuras de datos y esquemas
/// utilizados en toda la aplicación. Define entidades como Usuario,
/// Estudiante, Curso, etc., y sus relaciones.
pub mod models;

/// Módulo que define las rutas HTTP y endpoints de la API REST.
/// Maneja las solicitudes entrantes, la validación de parámetros
/// y la conexión con los servicios correspondientes.
pub mod routes;

/// Módulo que implementa la lógica de negocio de la aplicación.
/// Contiene los servicios que procesan datos, realizan cálculos
/// y operaciones sobre las entidades del sistema.
pub mod services;

/// Módulo que proporciona funciones auxiliares y utilidades
/// reutilizables en diferentes partes de la aplicación, como
/// formateo de datos, validaciones, manejo de fechas y más.
pub mod utils;

// Re-exportaciones para facilitar el uso de la API
pub use models::*;
pub use routes::*;
pub use services::*;
pub use utils::*;

