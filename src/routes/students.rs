use actix_web::{
    delete, get, post, put,
    web::{self, Data, Json, Path},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    models::student::Student,
    services::students::StudentService,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateStudentRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub user_id: Option<Uuid>,
    pub phone: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateStudentRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}

#[get("")]
async fn get_all_students(student_service: Data<StudentService>) -> impl Responder {
    match student_service.get_all_students().await {
        Ok(students) => HttpResponse::Ok().json(students),
        Err(e) => {
            log::error!("Failed to get all students: {}", e);
            HttpResponse::InternalServerError().json(format!("Failed to get students: {}", e))
        }
    }
}

#[get("/{id}")]
async fn get_student_by_id(
    path: Path<(Uuid,)>,
    student_service: Data<StudentService>,
) -> impl Responder {
    let id = path.into_inner().0;
    match student_service.get_student_by_id(id).await {
        Ok(Some(student)) => HttpResponse::Ok().json(student),
        Ok(None) => HttpResponse::NotFound().json("Student not found"),
        Err(e) => {
            log::error!("Failed to get student by id: {}", e);
            HttpResponse::InternalServerError().json(format!("Failed to get student: {}", e))
        }
    }
}

#[post("")]
async fn create_student(
    req: Json<CreateStudentRequest>,
    student_service: Data<StudentService>,
) -> impl Responder {
    match student_service.create_student(req.into_inner()).await {
        Ok(student) => HttpResponse::Created().json(student),
        Err(e) => {
            log::error!("Failed to create student: {}", e);
            HttpResponse::InternalServerError().json(format!("Failed to create student: {}", e))
        }
    }
}

#[put("/{id}")]
async fn update_student(
    path: Path<(Uuid,)>,
    req: Json<UpdateStudentRequest>,
    student_service: Data<StudentService>,
) -> impl Responder {
    let id = path.into_inner().0;
    match student_service.update_student(id, req.into_inner()).await {
        Ok(Some(student)) => HttpResponse::Ok().json(student),
        Ok(None) => HttpResponse::NotFound().json("Student not found"),
        Err(e) => {
            log::error!("Failed to update student: {}", e);
            HttpResponse::InternalServerError().json(format!("Failed to update student: {}", e))
        }
    }
}

#[delete("/{id}")]
async fn delete_student(
    path: Path<(Uuid,)>,
    student_service: Data<StudentService>,
) -> impl Responder {
    let id = path.into_inner().0;
    match student_service.delete_student(id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json("Student not found"),
        Err(e) => {
            log::error!("Failed to delete student: {}", e);
            HttpResponse::InternalServerError().json(format!("Failed to delete student: {}", e))
        }
    }
}

pub fn routes() -> actix_web::Scope {
    web::scope("/students")
        .service(get_all_students)
        .service(get_student_by_id)
        .service(create_student)
        .service(update_student)
        .service(delete_student)
}

