use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use dotenv::dotenv;
use std::env;
use log::{info, error};

// Importamos nuestra biblioteca sai
use sai::{models, routes, services, utils, db};

// Estructura para configuración de la aplicación
struct AppState {
    db_pool: db::DbPool,
}

// Manejador simple para la ruta principal
async fn index() -> impl Responder {
    HttpResponse::Ok().body("¡Bienvenido al Sistema Administrativo Integral (SAI)!")
}

// Manejador para verificar el estado del servidor
async fn health_check(data: web::Data<AppState>) -> impl Responder {
    // Usamos el método de verificación de conexión de nuestro módulo db
    match db::helpers::transaction(&data.db_pool, |_tx| Box::pin(async { 
        Ok::<_, sqlx::Error>(sqlx::query("SELECT 1").execute(&data.db_pool).await?)
    })).await {
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
    
    // Inicializar la conexión a la base de datos usando nuestro módulo db
    // Esto incluye verificación de conexión e inicialización del esquema si es necesario
    let pool = db::initialize_db().await;
    
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
            // Register API routes with database pool available to all routes
            .service(web::scope("")
                .app_data(web::Data::clone(&web::Data::new(AppState {
                    db_pool: pool.clone(),
                })))
                .service(routes::configure())
                .service(routes::configure_system_routes())
            )
    })
    .bind(&server_url)?
    .run()
    .await
}
