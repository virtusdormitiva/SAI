use actix_web::{
    delete, get, post, put,
    web::{self, Data, Json, Path},
    HttpResponse, Responder, Scope,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    models::teacher::Teacher,
    services::teachers::{CreateTeacherError, TeacherService, UpdateTeacherError},
};

#[derive(Debug, Serialize)]
pub struct TeacherResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub specialization: String,
    pub hire_date: chrono::NaiveDate,
    pub department: String,
    pub is_active: bool,
}

impl From<Teacher> for TeacherResponse {
    fn from(teacher: Teacher) -> Self {
        Self {
            id: teacher.id,
            user_id: teacher.user_id,
            specialization: teacher.specialization,
            hire_date: teacher.hire_date,
            department: teacher.department,
            is_active: teacher.is_active,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateTeacherRequest {
    pub user_id: Uuid,
    pub specialization: String,
    pub hire_date: chrono::NaiveDate,
    pub department: String,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTeacherRequest {
    pub specialization: Option<String>,
    pub hire_date: Option<chrono::NaiveDate>,
    pub department: Option<String>,
    pub is_active: Option<bool>,
}

#[get("")]
async fn get_all_teachers(service: Data<TeacherService>) -> impl Responder {
    match service.get_all_teachers().await {
        Ok(teachers) => {
            let teacher_responses: Vec<TeacherResponse> = teachers.into_iter().map(TeacherResponse::from).collect();
            HttpResponse::Ok().json(teacher_responses)
        }
        Err(err) => {
            log::error!("Failed to get all teachers: {:?}", err);
            HttpResponse::InternalServerError().json("Failed to get teachers")
        }
    }
}

#[get("/{id}")]
async fn get_teacher_by_id(path: Path<Uuid>, service: Data<TeacherService>) -> impl Responder {
    let teacher_id = path.into_inner();
    
    match service.get_teacher_by_id(teacher_id).await {
        Ok(Some(teacher)) => HttpResponse::Ok().json(TeacherResponse::from(teacher)),
        Ok(None) => HttpResponse::NotFound().json("Teacher not found"),
        Err(err) => {
            log::error!("Failed to get teacher {}: {:?}", teacher_id, err);
            HttpResponse::InternalServerError().json("Failed to get teacher")
        }
    }
}

#[post("")]
async fn create_teacher(
    request: Json<CreateTeacherRequest>,
    service: Data<TeacherService>,
) -> impl Responder {
    match service.create_teacher(
        request.user_id,
        request.specialization.clone(),
        request.hire_date,
        request.department.clone(),
        request.is_active,
    ).await {
        Ok(teacher) => HttpResponse::Created().json(TeacherResponse::from(teacher)),
        Err(CreateTeacherError::UserNotFound) => {
            HttpResponse::BadRequest().json("User not found")
        }
        Err(CreateTeacherError::UserAlreadyAssigned) => {
            HttpResponse::BadRequest().json("User is already assigned to a teacher")
        }
        Err(CreateTeacherError::DatabaseError(err)) => {
            log::error!("Failed to create teacher: {:?}", err);
            HttpResponse::InternalServerError().json("Failed to create teacher")
        }
    }
}

#[put("/{id}")]
async fn update_teacher(
    path: Path<Uuid>,
    request: Json<UpdateTeacherRequest>,
    service: Data<TeacherService>,
) -> impl Responder {
    let teacher_id = path.into_inner();
    
    match service.update_teacher(
        teacher_id,
        request.specialization.clone(),
        request.hire_date,
        request.department.clone(),
        request.is_active,
    ).await {
        Ok(teacher) => HttpResponse::Ok().json(TeacherResponse::from(teacher)),
        Err(UpdateTeacherError::TeacherNotFound) => {
            HttpResponse::NotFound().json("Teacher not found")
        }
        Err(UpdateTeacherError::DatabaseError(err)) => {
            log::error!("Failed to update teacher {}: {:?}", teacher_id, err);
            HttpResponse::InternalServerError().json("Failed to update teacher")
        }
    }
}

#[delete("/{id}")]
async fn delete_teacher(path: Path<Uuid>, service: Data<TeacherService>) -> impl Responder {
    let teacher_id = path.into_inner();
    
    match service.delete_teacher(teacher_id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json("Teacher not found"),
        Err(err) => {
            log::error!("Failed to delete teacher {}: {:?}", teacher_id, err);
            HttpResponse::InternalServerError().json("Failed to delete teacher")
        }
    }
}

pub fn routes() -> Scope {
    web::scope("/teachers")
        .service(get_all_teachers)
        .service(get_teacher_by_id)
        .service(create_teacher)
        .service(update_teacher)
        .service(delete_teacher)
}

