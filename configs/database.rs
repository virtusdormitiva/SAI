use dotenv::dotenv;
use std::env;
use std::time::Duration;
use tokio_postgres::{Config, Error as PgError, NoTls};
use deadpool_postgres::{Config as PoolConfig, Pool, PoolError, Runtime};
use serde::{Deserialize, Serialize};

/// Representa la configuración de la base de datos PostgreSQL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Nombre de host o dirección IP del servidor
    pub host: String,
    /// Puerto de conexión
    pub port: u16,
    /// Nombre de la base de datos
    pub dbname: String,
    /// Nombre de usuario para autenticación
    pub username: String,
    /// Contraseña para autenticación
    pub password: String,
    /// Número máximo de conexiones en el pool
    pub max_connections: u32,
    /// Tiempo de espera para operaciones de conexión (en segundos)
    pub connection_timeout: u64,
    /// Tiempo de espera para operaciones inactivas (en segundos)
    pub idle_timeout: u64,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 5432,
            dbname: "sai_db".to_string(),
            username: "postgres".to_string(),
            password: "postgres".to_string(),
            max_connections: 20,
            connection_timeout: 30,
            idle_timeout: 600,
        }
    }
}

impl DatabaseConfig {
    /// Carga la configuración de la base de datos desde variables de entorno.
    ///
    /// # Ejemplo
    ///
    /// ```
    /// let db_config = DatabaseConfig::from_env();
    /// ```
    pub fn from_env() -> Self {
        dotenv().ok();

        Self {
            host: env::var("DATABASE_HOST").unwrap_or_else(|_| "localhost".to_string()),
            port: env::var("DATABASE_PORT")
                .unwrap_or_else(|_| "5432".to_string())
                .parse()
                .unwrap_or(5432),
            dbname: env::var("DATABASE_NAME").unwrap_or_else(|_| "sai_db".to_string()),
            username: env::var("DATABASE_USER").unwrap_or_else(|_| "postgres".to_string()),
            password: env::var("DATABASE_PASSWORD").unwrap_or_else(|_| "postgres".to_string()),
            max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "20".to_string())
                .parse()
                .unwrap_or(20),
            connection_timeout: env::var("DATABASE_CONNECTION_TIMEOUT")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
            idle_timeout: env::var("DATABASE_IDLE_TIMEOUT")
                .unwrap_or_else(|_| "600".to_string())
                .parse()
                .unwrap_or(600),
        }
    }

    /// Crea una cadena de conexión a partir de la configuración.
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.dbname
        )
    }

    /// Convierte la configuración a un objeto Config de tokio_postgres.
    pub fn to_pg_config(&self) -> Config {
        let mut config = Config::new();
        config
            .host(&self.host)
            .port(self.port)
            .dbname(&self.dbname)
            .user(&self.username)
            .password(&self.password)
            .connect_timeout(Duration::from_secs(self.connection_timeout));
        config
    }

    /// Crea un pool de conexiones a partir de la configuración.
    pub fn create_pool(&self) -> Result<Pool, PoolError> {
        let mut pg_config = PoolConfig::new();
        pg_config.host = Some(self.host.clone());
        pg_config.port = Some(self.port);
        pg_config.dbname = Some(self.dbname.clone());
        pg_config.user = Some(self.username.clone());
        pg_config.password = Some(self.password.clone());
        pg_config.max_size = self.max_connections;

        Pool::builder(pg_config)
            .runtime(Runtime::Tokio1)
            .build()
    }
}

/// Errores que pueden ocurrir durante operaciones de base de datos.
#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    /// Error de conexión a PostgreSQL
    #[error("Error de conexión PostgreSQL: {0}")]
    ConnectionError(#[from] PgError),
    
    /// Error del pool de conexiones
    #[error("Error del pool de conexiones: {0}")]
    PoolError(#[from] PoolError),
    
    /// Error de configuración
    #[error("Error de configuración: {0}")]
    ConfigError(String),
}

/// Inicializa una conexión a la base de datos.
///
/// # Ejemplos
///
/// ```
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let db_config = DatabaseConfig::from_env();
/// let pool = init_database_pool(&db_config)?;
/// # Ok(())
/// # }
/// ```
pub fn init_database_pool(config: &DatabaseConfig) -> Result<Pool, DatabaseError> {
    config.create_pool().map_err(DatabaseError::PoolError)
}

/// Verifica la conexión a la base de datos.
///
/// # Ejemplos
///
/// ```
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let db_config = DatabaseConfig::from_env();
/// let pool = init_database_pool(&db_config)?;
/// check_database_connection(&pool).await?;
/// # Ok(())
/// # }
/// ```
pub async fn check_database_connection(pool: &Pool) -> Result<(), DatabaseError> {
    let client = pool.get().await.map_err(DatabaseError::PoolError)?;
    let result = client.query_one("SELECT 1", &[]).await.map_err(DatabaseError::ConnectionError)?;
    
    let value: i32 = result.get(0);
    if value == 1 {
        Ok(())
    } else {
        Err(DatabaseError::ConfigError("Error al verificar la conexión a la base de datos".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = DatabaseConfig::default();
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 5432);
        assert_eq!(config.max_connections, 20);
    }

    #[test]
    fn test_connection_string() {
        let config = DatabaseConfig {
            host: "db.example.com".to_string(),
            port: 5432,
            dbname: "testdb".to_string(),
            username: "testuser".to_string(),
            password: "testpass".to_string(),
            max_connections: 10,
            connection_timeout: 30,
            idle_timeout: 600,
        };

        assert_eq!(
            config.connection_string(),
            "postgres://testuser:testpass@db.example.com:5432/testdb"
        );
    }
}

