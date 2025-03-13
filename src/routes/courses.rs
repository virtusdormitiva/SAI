use actix_web::{
    delete, get, post, put,
    web::{self, Data, Json, Path},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    models::course::{Course, NewCourse, UpdateCourse},
    services::courses::CourseService,
};

#[get("")]
async fn get_all_courses(course_service: Data<CourseService>) -> impl Responder {
    match course_service.get_all_courses().await {
        Ok(courses) => HttpResponse::Ok().json(courses),
        Err(e) => {
            log::error!("Failed to get courses: {}", e);
            HttpResponse::InternalServerError().json("Failed to get courses")
        }
    }
}

#[get("/{id}")]
async fn get_course_by_id(
    path: Path<(Uuid,)>,
    course_service: Data<CourseService>,
) -> impl Responder {
    let course_id = path.into_inner().0;
    
    match course_service.get_course_by_id(course_id).await {
        Ok(Some(course)) => HttpResponse::Ok().json(course),
        Ok(None) => HttpResponse::NotFound().json("Course not found"),
        Err(e) => {
            log::error!("Failed to get course: {}", e);
            HttpResponse::InternalServerError().json("Failed to get course")
        }
    }
}

#[post("")]
async fn create_course(
    course: Json<NewCourse>,
    course_service: Data<CourseService>,
) -> impl Responder {
    match course_service.create_course(course.into_inner()).await {
        Ok(course) => HttpResponse::Created().json(course),
        Err(e) => {
            log::error!("Failed to create course: {}", e);
            HttpResponse::InternalServerError().json("Failed to create course")
        }
    }
}

#[put("/{id}")]
async fn update_course(
    path: Path<(Uuid,)>,
    course: Json<UpdateCourse>,
    course_service: Data<CourseService>,
) -> impl Responder {
    let course_id = path.into_inner().0;
    
    match course_service.update_course(course_id, course.into_inner()).await {
        Ok(Some(updated_course)) => HttpResponse::Ok().json(updated_course),
        Ok(None) => HttpResponse::NotFound().json("Course not found"),
        Err(e) => {
            log::error!("Failed to update course: {}", e);
            HttpResponse::InternalServerError().json("Failed to update course")
        }
    }
}

#[delete("/{id}")]
async fn delete_course(
    path: Path<(Uuid,)>,
    course_service: Data<CourseService>,
) -> impl Responder {
    let course_id = path.into_inner().0;
    
    match course_service.delete_course(course_id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json("Course not found"),
        Err(e) => {
            log::error!("Failed to delete course: {}", e);
            HttpResponse::InternalServerError().json("Failed to delete course")
        }
    }
}

#[get("/academic-year/{year}")]
async fn get_courses_by_academic_year(
    path: Path<(String,)>,
    course_service: Data<CourseService>,
) -> impl Responder {
    let academic_year = path.into_inner().0;
    
    match course_service.get_courses_by_academic_year(&academic_year).await {
        Ok(courses) => HttpResponse::Ok().json(courses),
        Err(e) => {
            log::error!("Failed to get courses by academic year: {}", e);
            HttpResponse::InternalServerError().json("Failed to get courses by academic year")
        }
    }
}

#[get("/stats/academic-year")]
async fn get_stats_by_academic_year(
    course_service: Data<CourseService>,
) -> impl Responder {
    match course_service.stats_by_academic_year().await {
        Ok(stats) => HttpResponse::Ok().json(stats),
        Err(e) => {
            log::error!("Failed to get course stats: {}", e);
            HttpResponse::InternalServerError().json("Failed to get course stats")
        }
    }
}

pub fn routes() -> actix_web::Scope {
    web::scope("/courses")
        .service(get_all_courses)
        .service(get_course_by_id)
        .service(create_course)
        .service(update_course)
        .service(delete_course)
        .service(get_courses_by_academic_year)
        .service(get_stats_by_academic_year)
}

