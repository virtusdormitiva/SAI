use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use dotenv::dotenv;
use std::env;
use sqlx::{PgPool, postgres::PgPoolOptions};
use log::{info, error};

// Importamos nuestra biblioteca sai
use sai::{models, routes, services, utils};

// Estructura para configuración de la aplicación
struct AppState {
    db_pool: PgPool,
}

// Manejador simple para la ruta principal
async fn index() -> impl Responder {
    HttpResponse::Ok().body("¡Bienvenido al Sistema Administrativo Integral (SAI)!")
}

// Manejador para verificar el estado del servidor
async fn health_check(data: web::Data<AppState>) -> impl Responder {
    // Intentamos hacer una consulta simple para verificar la conexión a la base de datos
    match sqlx::query("SELECT 1").execute(&data.db_pool).await {
        Ok(_) => HttpResponse::Ok().body("¡El servidor está en funcionamiento y conectado a la base de datos!"),
        Err(e) => {
            error!("Error al verificar la conexión a la base de datos: {}", e);
            HttpResponse::InternalServerError().body("Error de conexión a la base de datos")
        }
    }
}

// Función principal que configura y ejecuta el servidor
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Configuración de variables de entorno
    dotenv().ok();
    
    // Inicializar el logger
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    
    // Obtener la URL de la base de datos desde las variables de entorno
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    
    // Configuración del pool de conexiones a la base de datos
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await {
            Ok(pool) => {
                info!("Conexión exitosa a la base de datos");
                pool
            },
            Err(e) => {
                error!("Error al conectar a la base de datos: {}", e);
                panic!("No se pudo establecer conexión con la base de datos");
            }
        };
    
    // Dirección del servidor
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let server_url = format!("{}:{}", host, port);
    
    info!("Iniciando servidor en http://{}", server_url);
    
    // Configuración y ejecución del servidor
    HttpServer::new(move || {
        App::new()
            // Compartir el estado de la aplicación con los manejadores
            .app_data(web::Data::new(AppState {
                db_pool: pool.clone(),
            }))
            // Configuración de rutas básicas
            .route("/", web::get().to(index))
            .route("/health", web::get().to(health_check))
            // Register API routes
            .service(routes::configure())
            .service(routes::configure_system_routes())
    })
    .bind(&server_url)?
    .run()
    .await
}
