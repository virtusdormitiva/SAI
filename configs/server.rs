use std::env;
use std::time::Duration;

/// Estructura de configuración para el servidor HTTP
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Dirección IP donde escuchará el servidor
    pub host: String,
    /// Puerto donde escuchará el servidor
    pub port: u16,
    /// Tamaño máximo del payload en bytes
    pub max_payload_size: usize,
    /// Duración del timeout para conexiones
    pub timeout: Duration,
    /// Habilitar o deshabilitar CORS
    pub cors_enabled: bool,
    /// Orígenes permitidos para CORS (solo se usa si cors_enabled es true)
    pub cors_origins: Vec<String>,
    /// Métodos permitidos para CORS
    pub cors_methods: Vec<String>,
    /// Headers permitidos para CORS
    pub cors_allowed_headers: Vec<String>,
    /// Headers expuestos en CORS
    pub cors_exposed_headers: Vec<String>,
    /// Permitir credenciales en CORS
    pub cors_allow_credentials: bool,
    /// Tiempo máximo de cache para preflight CORS en segundos
    pub cors_max_age: u32,
    /// Habilitar o deshabilitar HTTPS
    pub https_enabled: bool,
    /// Ruta al certificado SSL (solo se usa si https_enabled es true)
    pub ssl_cert_path: Option<String>,
    /// Ruta a la clave privada SSL (solo se usa si https_enabled es true)
    pub ssl_key_path: Option<String>,
    /// Número de workers para el servidor
    pub workers: usize,
    /// Habilitar o deshabilitar la compresión
    pub enable_compression: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
            max_payload_size: 2 * 1024 * 1024, // 2 MB
            timeout: Duration::from_secs(30),
            cors_enabled: true,
            cors_origins: vec!["*".to_string()],
            cors_methods: vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "DELETE".to_string(),
                "OPTIONS".to_string(),
                "PATCH".to_string(),
            ],
            cors_allowed_headers: vec![
                "Origin".to_string(),
                "X-Requested-With".to_string(),
                "Content-Type".to_string(),
                "Accept".to_string(),
                "Authorization".to_string(),
            ],
            cors_exposed_headers: vec![
                "Content-Length".to_string(),
                "Content-Disposition".to_string(),
            ],
            cors_allow_credentials: true,
            cors_max_age: 3600,
            https_enabled: false,
            ssl_cert_path: None,
            ssl_key_path: None,
            workers: num_cpus::get(),
            enable_compression: true,
        }
    }
}

impl ServerConfig {
    /// Carga la configuración del servidor desde variables de entorno
    pub fn from_env() -> Self {
        let mut config = ServerConfig::default();

        // Configuración básica del servidor
        if let Ok(host) = env::var("SERVER_HOST") {
            config.host = host;
        }

        if let Ok(port) = env::var("SERVER_PORT") {
            if let Ok(port) = port.parse::<u16>() {
                config.port = port;
            }
        }

        if let Ok(max_size) = env::var("SERVER_MAX_PAYLOAD_SIZE") {
            if let Ok(max_size) = max_size.parse::<usize>() {
                config.max_payload_size = max_size;
            }
        }

        if let Ok(timeout) = env::var("SERVER_TIMEOUT") {
            if let Ok(timeout) = timeout.parse::<u64>() {
                config.timeout = Duration::from_secs(timeout);
            }
        }

        // Configuración de workers
        if let Ok(workers) = env::var("SERVER_WORKERS") {
            if let Ok(workers) = workers.parse::<usize>() {
                config.workers = workers;
            }
        }

        // Configuración de compresión
        if let Ok(enable_compression) = env::var("SERVER_ENABLE_COMPRESSION") {
            config.enable_compression = enable_compression.to_lowercase() == "true";
        }

        // Configuración CORS
        if let Ok(cors_enabled) = env::var("CORS_ENABLED") {
            config.cors_enabled = cors_enabled.to_lowercase() == "true";
        }

        if let Ok(cors_origins) = env::var("CORS_ORIGINS") {
            config.cors_origins = cors_origins
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
        }

        if let Ok(cors_methods) = env::var("CORS_METHODS") {
            config.cors_methods = cors_methods
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
        }

        if let Ok(cors_headers) = env::var("CORS_ALLOWED_HEADERS") {
            config.cors_allowed_headers = cors_headers
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
        }

        if let Ok(cors_exposed) = env::var("CORS_EXPOSED_HEADERS") {
            config.cors_exposed_headers = cors_exposed
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
        }

        if let Ok(cors_credentials) = env::var("CORS_ALLOW_CREDENTIALS") {
            config.cors_allow_credentials = cors_credentials.to_lowercase() == "true";
        }

        if let Ok(cors_max_age) = env::var("CORS_MAX_AGE") {
            if let Ok(max_age) = cors_max_age.parse::<u32>() {
                config.cors_max_age = max_age;
            }
        }

        // Configuración HTTPS
        if let Ok(https_enabled) = env::var("HTTPS_ENABLED") {
            config.https_enabled = https_enabled.to_lowercase() == "true";
        }

        if config.https_enabled {
            if let Ok(cert_path) = env::var("SSL_CERT_PATH") {
                config.ssl_cert_path = Some(cert_path);
            }

            if let Ok(key_path) = env::var("SSL_KEY_PATH") {
                config.ssl_key_path = Some(key_path);
            }
        }

        config
    }

    /// Retorna la dirección completa del servidor (host:puerto)
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    /// Valida que la configuración sea correcta
    pub fn validate(&self) -> Result<(), String> {
        if self.https_enabled {
            if self.ssl_cert_path.is_none() {
                return Err("SSL_CERT_PATH es requerido cuando HTTPS está habilitado".to_string());
            }
            if self.ssl_key_path.is_none() {
                return Err("SSL_KEY_PATH es requerido cuando HTTPS está habilitado".to_string());
            }
        }

        Ok(())
    }

    /// Configura el CORS para un servidor Actix-web
    #[cfg(feature = "actix-web")]
    pub fn configure_cors(&self) -> actix_cors::Cors {
        let mut cors = actix_cors::Cors::default();

        if self.cors_enabled {
            // Configurar orígenes permitidos
            if self.cors_origins.contains(&"*".to_string()) {
                cors = cors.allow_any_origin();
            } else {
                for origin in &self.cors_origins {
                    cors = cors.allowed_origin(origin);
                }
            }

            // Configurar métodos permitidos
            for method in &self.cors_methods {
                cors = cors.allowed_method(method);
            }

            // Configurar headers permitidos
            for header in &self.cors_allowed_headers {
                cors = cors.allowed_header(header);
            }

            // Configurar headers expuestos
            for header in &self.cors_exposed_headers {
                cors = cors.expose_header(header);
            }

            // Configurar otras opciones de CORS
            cors = cors
                .supports_credentials(self.cors_allow_credentials)
                .max_age(self.cors_max_age);
        }

        cors
    }
}

