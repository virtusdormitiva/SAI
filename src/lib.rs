//! Sistema Administrativo Integral (SAI) Library
//! 
//! This library provides core modules for the SAI application,
//! including models, routes, services, utilities, and database handling.

pub mod models;
pub mod routes;
pub mod services;
pub mod utils;
pub mod db;

// Re-export common items for easier imports
pub use models::*;
pub use routes::*;
pub use db::DbPool;

/// Application configuration constants
pub mod config {
    /// Default database connection URL
    pub const DEFAULT_DB_URL: &str = "postgres://postgres:postgres@localhost/sai";
    
    /// Default server address
    pub const DEFAULT_SERVER_ADDR: &str = "127.0.0.1:8080";
    
    /// Default log level
    pub const DEFAULT_LOG_LEVEL: &str = "info";
}

/// Initialize logging for the application
pub fn init_logger() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
}

/// Version information
pub mod version {
    /// Current version of the SAI application
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");
    
    /// Returns a formatted version string with build information
    pub fn get_version_string() -> String {
        format!("SAI v{}", VERSION)
    }
}

//! # Sistema Administrativo Integral (SAI)
//!
//! `sai` es una biblioteca diseñada para facilitar la gestión administrativa
//! integral de instituciones educativas y otras organizaciones en Paraguay.
//! Implementa funcionalidades que cumplen con normativas locales y estándares
//! de seguridad para el manejo de datos.
//!
//! Los módulos principales son:
//! - `models`: Estructuras de datos y esquemas
//! - `routes`: Endpoints de la API REST
//! - `services`: Lógica de negocio
//! - `utils`: Funciones auxiliares
//! - `db`: Gestión de la base de datos

// Re-exportaciones adicionales para facilitar el uso de la API
pub use services::*;
pub use utils::*;

