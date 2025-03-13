//! # Módulo de Utilidades
//! 
//! Este módulo proporciona funciones y herramientas de utilidad para todo el sistema SAI,
//! adaptadas específicamente para el contexto paraguayo.
//! 
//! Incluye funcionalidades para:
//! * Validación de documentos y datos paraguayos
//! * Formateo de datos según estándares locales
//! * Manejo de fechas y cálculos temporales
//! * Generación de identificadores únicos
//! * Utilidades para manejo de moneda (guaraníes)
//! * Otras funciones de utilidad general

pub mod validation;
pub mod formatting;
pub mod date_utils;
pub mod id_generator;
pub mod currency;
pub mod string_utils;

// Re-exportamos las funciones más utilizadas para facilitar su uso
pub use validation::{validate_ci, validate_ruc, validate_phone_number};
pub use formatting::{format_ci, format_ruc, format_phone_number};
pub use date_utils::{format_date_py, is_paraguay_holiday};
pub use currency::{format_guaranies, guaranies_to_words};

/// Constantes de utilidad general para el contexto paraguayo
pub mod constants {
    /// Código de país para Paraguay (según ISO 3166-1)
    pub const COUNTRY_CODE: &str = "PY";
    
    /// Código de llamada internacional para Paraguay
    pub const PHONE_COUNTRY_CODE: &str = "+595";
    
    /// Prefijos de operadoras móviles paraguayas
    pub const MOBILE_PREFIXES: [&str; 3] = ["9", "98", "99"];
    
    /// Longitud estándar de un CI paraguayo (sin puntos)
    pub const CI_LENGTH: usize = 7;
    
    /// Longitud estándar de un RUC paraguayo (sin guión)
    pub const RUC_BASE_LENGTH: usize = 8;
}

/// Módulo de validación de documentos y datos paraguayos
pub mod validation {
    use super::constants::*;
    use regex::Regex;
    
    /// Valida un número de Cédula de Identidad paraguaya
    /// 
    /// # Argumentos
    /// * `ci` - Número de cédula a validar (puede contener puntos)
    /// 
    /// # Ejemplos
    /// ```
    /// use sai::utils::validation::validate_ci;
    /// 
    /// assert!(validate_ci("1234567"));
    /// assert!(validate_ci("1.234.567"));
    /// assert!(!validate_ci("12345")); // Muy corto
    /// ```
    pub fn validate_ci(ci: &str) -> bool {
        let digits_only = ci.replace(".", "");
        
        // Verificar longitud básica
        if digits_only.len() != CI_LENGTH {
            return false;
        }
        
        // Verificar que solo contiene dígitos
        digits_only.chars().all(|c| c.is_digit(10))
    }
    
    /// Valida un número de RUC paraguayo
    /// 
    /// # Argumentos
    /// * `ruc` - Número de RUC a validar (puede contener guión y dígito verificador)
    /// 
    /// # Ejemplos
    /// ```
    /// use sai::utils::validation::validate_ruc;
    /// 
    /// assert!(validate_ruc("12345678-9"));
    /// assert!(validate_ruc("123456789"));
    /// assert!(!validate_ruc("1234-5")); // Formato incorrecto
    /// ```
    pub fn validate_ruc(ruc: &str) -> bool {
        // RUC puede tener formato XXXXXXXX-Y o XXXXXXXXY
        let ruc_regex = Regex::new(r"^(\d{7,8})[-]?(\d)$").unwrap();
        
        if !ruc_regex.is_match(ruc) {
            return false;
        }
        
        // TODO: Implementar algoritmo de verificación del dígito verificador
        // Para una implementación completa, se debe verificar que el último dígito
        // sea correcto según el algoritmo de verificación de RUC paraguayo.
        
        true
    }
    
    /// Valida un número de teléfono paraguayo
    /// 
    /// # Argumentos
    /// * `phone` - Número de teléfono a validar
    /// 
    /// # Ejemplos
    /// ```
    /// use sai::utils::validation::validate_phone_number;
    /// 
    /// assert!(validate_phone_number("0981123456"));
    /// assert!(validate_phone_number("+595981123456"));
    /// assert!(!validate_phone_number("123456")); // Muy corto
    /// ```
    pub fn validate_phone_number(phone: &str) -> bool {
        let phone_clean = phone
            .replace(" ", "")
            .replace("-", "")
            .replace("(", "")
            .replace(")", "");
            
        // Formato local o internacional
        if phone_clean.starts_with(PHONE_COUNTRY_CODE) {
            // Formato internacional +595XXXXXXXXX
            phone_clean.len() >= 12 && phone_clean[4..].chars().all(|c| c.is_digit(10))
        } else if phone_clean.starts_with("0") {
            // Formato local 0XXXXXXXXX
            phone_clean.len() >= 10 && phone_clean.chars().all(|c| c.is_digit(10))
        } else {
            false
        }
    }
    
    /// Valida una dirección de correo electrónico
    pub fn validate_email(email: &str) -> bool {
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        email_regex.is_match(email)
    }
}

/// Módulo para formateo de datos según estándares locales paraguayos
pub mod formatting {
    /// Formatea un número de Cédula de Identidad con el formato paraguayo
    /// 
    /// # Argumentos
    /// * `ci` - Número de cédula sin formato
    /// 
    /// # Ejemplos
    /// ```
    /// use sai::utils::formatting::format_ci;
    /// 
    /// assert_eq!(format_ci("1234567"), "1.234.567");
    /// ```
    pub fn format_ci(ci: &str) -> String {
        let digits_only = ci.replace(".", "");
        
        match digits_only.len() {
            7 => format!(
                "{}.{}.{}",
                &digits_only[0..1],
                &digits_only[1..4],
                &digits_only[4..7]
            ),
            6 => format!(
                "{}.{}.{}",
                &digits_only[0..1],
                &digits_only[1..3],
                &digits_only[3..6]
            ),
            _ => digits_only, // Devolver sin cambios si no coincide con el formato esperado
        }
    }
    
    /// Formatea un número de RUC con el formato paraguayo
    /// 
    /// # Argumentos
    /// * `ruc` - Número de RUC sin formato o parcialmente formateado
    /// 
    /// # Ejemplos
    /// ```
    /// use sai::utils::formatting::format_ruc;
    /// 
    /// assert_eq!(format_ruc("123456789"), "12345678-9");
    /// ```
    pub fn format_ruc(ruc: &str) -> String {
        let digits_only: String = ruc.chars().filter(|c| c.is_digit(10)).collect();
        
        if digits_only.len() < 2 {
            return digits_only;
        }
        
        let base = &digits_only[0..digits_only.len()-1];
        let check_digit = &digits_only[digits_only.len()-1..];
        
        format!("{}-{}", base, check_digit)
    }
    
    /// Formatea un número de teléfono con el formato paraguayo
    /// 
    /// # Argumentos
    /// * `phone` - Número de teléfono sin formato
    /// * `international` - Si es true, incluye el código de país (+595)
    /// 
    /// # Ejemplos
    /// ```
    /// use sai::utils::formatting::format_phone_number;
    /// 
    /// assert_eq!(format_phone_number("0981123456", false), "0981 123 456");
    /// assert_eq!(format_phone_number("981123456", true), "+595 981 123 456");
    /// ```
    pub fn format_phone_number(phone: &str, international: bool) -> String {
        let digits_only: String = phone
            .chars()
            .filter(|c| c.is_digit(10))
            .collect();
            
        // Eliminar el 0 inicial si existe y está en formato internacional
        let digits = if international && digits_only.starts_with('0') {
            digits_only[1..].to_string()
        } else {
            digits_only
        };
        
        // Formatear según longitud
        if digits.len() >= 9 {
            let area_code = &digits[..3];
            let middle = &digits[3..6];
            let end = &digits[6..9];
            
            if international {
                format!("+595 {} {} {}", area_code, middle, end)
            } else {
                format!("0{} {} {}", area_code, middle, end)
            }
        } else {
            // Si no cumple con la longitud esperada, devolver sin cambios
            if international && !phone.contains("+595") {
                format!("+595 {}", phone)
            } else {
                phone.to_string()
            }
        }
    }
}

/// Módulo para manejo de fechas según contexto paraguayo
pub mod date_utils {
    use chrono::{NaiveDate, Datelike};
    
    /// Formatea una fecha según el formato paraguayo (DD/MM/YYYY)
    /// 
    /// # Argumentos
    /// * `date` - Fecha a formatear
    /// 
    /// # Ejemplos
    /// ```
    /// use chrono::NaiveDate;
    /// use sai::utils::date_utils::format_date_py;
    /// 
    /// let date = NaiveDate::from_ymd_opt(2023, 5, 15).unwrap();
    /// assert_eq!(format_date_py(&date), "15/05/2023");
    /// ```
    pub fn format_date_py(date: &NaiveDate) -> String {
        format!("{:02}/{:02}/{:04}", date.day(), date.month(), date.year())
    }
    
    /// Verifica si una fecha es un feriado en Paraguay
    /// 
    /// # Argumentos
    /// * `date` - Fecha a verificar
    pub fn is_paraguay_holiday(date: &NaiveDate) -> bool {
        let (day, month, year) = (date.day(), date.month(), date.year());
        
        // Feriados fijos
        if (day == 1 && month == 1) ||    // Año Nuevo
           (day == 1 && month == 5) ||    // Día del Trabajador
           (day == 15 && month == 5) ||   // Independencia Nacional
           (day == 12 && month == 6) ||   // Paz del Chaco
           (day == 15 && month == 8) ||   // Fundación de Asunción
           (day == 29 && month == 9) ||   // Victoria de Boquerón
           (day == 8 && month == 12) ||   // Virgen de Caacupé
           (day == 25 && month == 12) {   // Navidad
            return true;
        }
        
        // TODO: Implementar cálculo de feriados móviles (Semana Santa, etc.)
        // Requiere algoritmos específicos para calcular fechas como Semana Santa
        
        false
    }
    
    /// Calcula la cantidad de días hábiles entre dos fechas
    /// 
    /// # Argumentos
    /// * `start_date` - Fecha de inicio
    /// * `end_date` - Fecha de fin
    pub fn business_days_between(start_date: &NaiveDate, end_date: &NaiveDate) -> u32 {
        let mut count = 0;
        let mut current_date = *start_date;
        
        while current_date <= *end_date {
            // Si no es fin de semana ni feriado
            if current_date.weekday().number_from_monday() <= 5 && !is_paraguay_holiday(&current_date) {
                count += 1;
            }
            current_date = current_date.succ_opt().unwrap_or(*end_date);
        }
        
        count
    }
}

/// Módulo para generación de identificadores únicos
pub mod id_generator {
    use uuid::Uuid;
    use chrono::{Utc, Datelike};
    
    /// Genera un UUID v4 para usar como identificador único
    pub fn generate_uuid() -> String {
        Uuid::new_v4().to_string()
    }
    
    /// Genera un código de estudiante basado en año y secuencia
    /// 
    /// # Argumentos
    /// * `sequence` - Número secuencial del estudiante
    /// 
    /// El formato es: E-YYYY-NNNNN donde YYYY es el año actual y NNNNN es el número secuencial
    pub fn generate_student_code(sequence: u32) -> String {
        let year = Utc::now().year();
        format!("E-{}-{:05}", year, sequence)
    }
    
    /// Genera un código de empleado basado en departamento y secuencia
    /// 
    /// # Argumentos
    /// * `department_code` - Código del departamento (2 caracteres)
    /// * `sequence` - Número secuencial del empleado
    pub fn generate_employee_code(department_code: &str, sequence: u32) -> String {
        format!("{}-{:04}", department_code.to_uppercase(), sequence)
    }
    
    /// Genera un código de factura según estándares paraguayos
    /// 
    /// # Argumentos
    /// * `branch` - Código de sucursal (

