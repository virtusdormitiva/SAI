use std::env;
use sqlx::{postgres::{PgPoolOptions, PgPool}, Pool, Postgres, Error as SqlxError};
use log::{info, error};
use dotenv::dotenv;

/// Type alias for PostgreSQL connection pool
pub type DbPool = Pool<Postgres>;

/// Database configuration parameters
#[derive(Debug, Clone)]
pub struct DbConfig {
    pub connection_string: String,
    pub max_connections: u32,
    pub acquire_timeout: std::time::Duration,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            connection_string: env::var("DATABASE_URL")
                .expect("DATABASE_URL environment variable not set"),
            max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .expect("DATABASE_MAX_CONNECTIONS must be a number"),
            acquire_timeout: std::time::Duration::from_secs(
                env::var("DATABASE_ACQUIRE_TIMEOUT")
                    .unwrap_or_else(|_| "30".to_string())
                    .parse()
                    .expect("DATABASE_ACQUIRE_TIMEOUT must be a number in seconds")
            ),
        }
    }
}

/// Database manager that handles connection pooling and operations
pub struct DbManager {
    pool: DbPool,
}

impl DbManager {
    /// Create a new database connection pool with the provided configuration
    pub async fn new(config: DbConfig) -> Result<Self, SqlxError> {
        let pool = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .acquire_timeout(config.acquire_timeout)
            .connect(&config.connection_string)
            .await?;
        
        info!("Database connection pool established with {} max connections", config.max_connections);
        
        Ok(Self { pool })
    }

    /// Create a new database connection pool with default configuration from environment variables
    pub async fn new_from_env() -> Result<Self, SqlxError> {
        dotenv().ok(); // Load environment variables from .env file if available
        let config = DbConfig::default();
        Self::new(config).await
    }

    /// Get a reference to the connection pool
    pub fn get_pool(&self) -> &DbPool {
        &self.pool
    }

    /// Check database connection by executing a simple query
    pub async fn check_connection(&self) -> Result<(), SqlxError> {
        sqlx::query("SELECT 1").execute(self.get_pool()).await?;
        info!("Database connection verified successfully");
        Ok(())
    }

    /// Initialize database with required schema if not already set up
    pub async fn initialize_schema(&self) -> Result<(), SqlxError> {
        info!("Checking and initializing database schema if needed");
        
        // Check if the migrations table exists, create it if not
        let migrations_table_exists = sqlx::query(
            "SELECT EXISTS (
                SELECT FROM information_schema.tables 
                WHERE table_schema = 'public' 
                AND table_name = 'schema_migrations'
            )"
        )
        .fetch_one(self.get_pool())
        .await?
        .get::<bool, _>(0);

        if !migrations_table_exists {
            info!("Creating schema_migrations table");
            sqlx::query(
                "CREATE TABLE schema_migrations (
                    version BIGINT PRIMARY KEY,
                    applied_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
                )"
            )
            .execute(self.get_pool())
            .await?;
        }

        info!("Database schema check completed");
        Ok(())
    }
}

/// Helper functions for common database operations
pub mod helpers {
    use super::*;
    use sqlx::{Transaction, Postgres, Row};
    use std::fmt::Debug;

    /// Execute a transaction with the provided closure
    pub async fn transaction<F, T, E>(pool: &DbPool, f: F) -> Result<T, E>
    where
        F: for<'a> FnOnce(&'a mut Transaction<'_, Postgres>) -> 
            std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, E>> + Send + 'a>>,
        E: From<SqlxError> + Debug,
    {
        let mut tx = pool.begin().await.map_err(|e| E::from(e))?;
        
        let result = match f(&mut tx).await {
            Ok(result) => {
                tx.commit().await.map_err(|e| E::from(e))?;
                Ok(result)
            }
            Err(e) => {
                if let Err(rollback_err) = tx.rollback().await {
                    error!("Failed to rollback transaction: {:?}", rollback_err);
                }
                Err(e)
            }
        };

        result
    }

    /// Check if a record exists in a table
    pub async fn record_exists(
        pool: &DbPool, 
        table: &str, 
        column: &str, 
        value: &str
    ) -> Result<bool, SqlxError> {
        let query = format!(
            "SELECT EXISTS(SELECT 1 FROM {} WHERE {} = $1)", 
            table, column
        );
        
        let result = sqlx::query(&query)
            .bind(value)
            .fetch_one(pool)
            .await?
            .get::<bool, _>(0);
            
        Ok(result)
    }

    /// Get the count of records in a table
    pub async fn count_records(
        pool: &DbPool, 
        table: &str, 
        condition: Option<&str>
    ) -> Result<i64, SqlxError> {
        let query = match condition {
            Some(cond) => format!("SELECT COUNT(*) FROM {} WHERE {}", table, cond),
            None => format!("SELECT COUNT(*) FROM {}", table),
        };
        
        let result = sqlx::query(&query)
            .fetch_one(pool)
            .await?
            .get::<i64, _>(0);
            
        Ok(result)
    }
}

/// Initialize the database connection pool for the application
pub async fn initialize_db() -> DbPool {
    match DbManager::new_from_env().await {
        Ok(manager) => {
            if let Err(e) = manager.check_connection().await {
                error!("Failed to verify database connection: {}", e);
                panic!("Database connection failed: {}", e);
            }

            if let Err(e) = manager.initialize_schema().await {
                error!("Failed to initialize database schema: {}", e);
                panic!("Database schema initialization failed: {}", e);
            }

            info!("Database initialized successfully");
            manager.get_pool().clone()
        }
        Err(e) => {
            error!("Failed to establish database connection: {}", e);
            panic!("Database connection failed: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[actix_rt::test]
    async fn test_db_config_default() {
        // Set up test environment variables
        env::set_var("DATABASE_URL", "postgres://test:test@localhost/testdb");
        env::set_var("DATABASE_MAX_CONNECTIONS", "5");
        env::set_var("DATABASE_ACQUIRE_TIMEOUT", "10");
        
        let config = DbConfig::default();
        
        assert_eq!(config.connection_string, "postgres://test:test@localhost/testdb");
        assert_eq!(config.max_connections, 5);
        assert_eq!(config.acquire_timeout, std::time::Duration::from_secs(10));
    }
    
    // Integration tests would need a test database
    // These are commented out since they require an actual database connection
    /*
    #[actix_rt::test]
    async fn test_connection_pool() {
        dotenv().ok();
        
        let manager = DbManager::new_from_env().await.expect("Failed to create pool");
        assert!(manager.check_connection().await.is_ok());
    }
    
    #[actix_rt::test]
    async fn test_record_exists() {
        dotenv().ok();
        
        let manager = DbManager::new_from_env().await.expect("Failed to create pool");
        let pool = manager.get_pool();
        
        // Set up test case - create a table and insert a record
        sqlx::query("CREATE TABLE IF NOT EXISTS test_table (id TEXT PRIMARY KEY)")
            .execute(pool)
            .await
            .expect("Failed to create test table");
            
        sqlx::query("INSERT INTO test_table (id) VALUES ('test_id') ON CONFLICT DO NOTHING")
            .execute(pool)
            .await
            .expect("Failed to insert test record");
            
        // Test the helper function
        let exists = helpers::record_exists(pool, "test_table", "id", "test_id")
            .await
            .expect("Failed to check if record exists");
            
        assert!(exists);
        
        // Clean up
        sqlx::query("DROP TABLE test_table")
            .execute(pool)
            .await
            .expect("Failed to drop test table");
    }
    */
}

