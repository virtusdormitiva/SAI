use actix_web::{
    web, HttpResponse, Responder, Error, HttpRequest, dev::HttpServiceFactory,
    http::StatusCode, guard,
};
use serde::{Deserialize, Serialize};
use crate::models::{
    user::{User, CreateUserDto, UpdateUserDto},
    student::{Student, CreateStudentDto, UpdateStudentDto},
    teacher::{Teacher, CreateTeacherDto, UpdateTeacherDto},
    course::{Course, CreateCourseDto, UpdateCourseDto},
};
use crate::services::{
    users::UserService,
    students::StudentService,
    teachers::TeacherService,
    courses::CourseService,
};
use crate::routes::auth::{Auth, Claims, TokenType};
use futures::future::{self, Future};
use std::sync::Arc;

// Role-based access middleware guard for admin routes
pub struct AdminGuard;

impl guard::Guard for AdminGuard {
    fn check(&self, req: &HttpRequest) -> bool {
        // First, check for Authorization header
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = auth_str.trim_start_matches("Bearer ").trim();
                    // Verify token using TokenType::Access enum variant for proper access token validation
                    match Auth::validate_token(token, TokenType::Access) {
                        Ok(claims) => {
                            // Explicitly verify that the user has admin role privileges
                            return claims.role == "admin";
                        }
                        Err(err) => {
                            // Log validation error for debugging
                            log::debug!("Token validation failed: {}", err);
                            return false;
                        }
                    }
                }
            }
        }

        // If there's no Authorization header, also check for auth cookie as fallback
        if let Some(cookie) = req.cookie("auth_token") {
            match Auth::validate_token(cookie.value(), TokenType::Access) {
                Ok(claims) => {
                    return claims.role == "admin";
                }
                Err(err) => {
                    log::debug!("Cookie token validation failed: {}", err);
                }
            }
        }

        // If no valid token found or user doesn't have admin role, deny access
        false
    }
}

// Response structures
#[derive(Serialize)]
struct AdminResponse<T> {
    success: bool,
    message: String,
    data: Option<T>,
}

// === USER MANAGEMENT ENDPOINTS ===

#[derive(Deserialize)]
struct UserQuery {
    page: Option<usize>,
    per_page: Option<usize>,
    search: Option<String>,
}

async fn get_all_users(
    query: web::Query<UserQuery>,
    user_service: web::Data<Arc<UserService>>,
) -> Result<impl Responder, Error> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);
    let search = query.search.clone();
    
    match user_service.get_all_users(page, per_page, search).await {
        Ok(users) => Ok(HttpResponse::Ok().json(AdminResponse {
            success: true,
            message: "Users retrieved successfully".to_string(),
            data: Some(users),
        })),
        Err(e) => Ok(HttpResponse::InternalServerError().json(AdminResponse::<Vec<User>> {
            success: false,
            message: format!("Failed to retrieve users: {}", e),
            data: None,
        }))
    }
}

async fn get_user_by_id(
    path: web::Path<String>,
    user_service: web::Data<Arc<UserService>>,
) -> Result<impl Responder, Error> {
    let id = path.into_inner();
    
    match user_service.get_user_by_id(&id).await {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(AdminResponse {
            success: true,
            message: "User retrieved successfully".to_string(),
            data: Some(user),
        })),
        Ok(None) => Ok(HttpResponse::NotFound().json(AdminResponse::<User> {
            success: false,
            message: "User not found".to_string(),
            data: None,
        })),
        Err(e) => Ok(HttpResponse::InternalServerError().json(AdminResponse::<User> {
            success: false,
            message: format!("Failed to retrieve user: {}", e),
            data: None,
        }))
    }
}

async fn create_user(
    user_dto: web::Json<CreateUserDto>,
    user_service: web::Data<Arc<UserService>>,
) -> Result<impl Responder, Error> {
    match user_service.create_user(user_dto.into_inner()).await {
        Ok(user) => Ok(HttpResponse::Created().json(AdminResponse {
            success: true,
            message: "User created successfully".to_string(),
            data: Some(user),
        })),
        Err(e) => Ok(HttpResponse::BadRequest().json(AdminResponse::<User> {
            success: false,
            message: format!("Failed to create user: {}", e),
            data: None,
        }))
    }
}

async fn update_user(
    path: web::Path<String>,
    user_dto: web::Json<UpdateUserDto>,
    user_service: web::Data<Arc<UserService>>,
) -> Result<impl Responder, Error> {
    let id = path.into_inner();
    
    match user_service.update_user(&id, user_dto.into_inner()).await {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(AdminResponse {
            success: true,
            message: "User updated successfully".to_string(),
            data: Some(user),
        })),
        Ok(None) => Ok(HttpResponse::NotFound().json(AdminResponse::<User> {
            success: false,
            message: "User not found".to_string(),
            data: None,
        })),
        Err(e) => Ok(HttpResponse::BadRequest().json(AdminResponse::<User> {
            success: false,
            message: format!("Failed to update user: {}", e),
            data: None,
        }))
    }
}

async fn delete_user(
    path: web::Path<String>,
    user_service: web::Data<Arc<UserService>>,
) -> Result<impl Responder, Error> {
    let id = path.into_inner();
    
    match user_service.delete_user(&id).await {
        Ok(true) => Ok(HttpResponse::Ok().json(AdminResponse::<()> {
            success: true,
            message: "User deleted successfully".to_string(),
            data: None,
        })),
        Ok(false) => Ok(HttpResponse::NotFound().json(AdminResponse::<()> {
            success: false,
            message: "User not found".to_string(),
            data: None,
        })),
        Err(e) => Ok(HttpResponse::InternalServerError().json(AdminResponse::<()> {
            success: false,
            message: format!("Failed to delete user: {}", e),
            data: None,
        }))
    }
}

// === STUDENT MANAGEMENT ENDPOINTS ===

#[derive(Deserialize)]
struct StudentQuery {
    page: Option<usize>,
    per_page: Option<usize>,
    search: Option<String>,
    course_id: Option<String>,
}

async fn get_all_students(
    query: web::Query<StudentQuery>,
    student_service: web::Data<Arc<StudentService>>,
) -> Result<impl Responder, Error> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);
    let search = query.search.clone();
    let course_id = query.course_id.clone();
    
    match student_service.get_all_students(page, per_page, search, course_id).await {
        Ok(students) => Ok(HttpResponse::Ok().json(AdminResponse {
            success: true,
            message: "Students retrieved successfully".to_string(),
            data: Some(students),
        })),
        Err(e) => Ok(HttpResponse::InternalServerError().json(AdminResponse::<Vec<Student>> {
            success: false,
            message: format!("Failed to retrieve students: {}", e),
            data: None,
        }))
    }
}

async fn get_student_by_id(
    path: web::Path<String>,
    student_service: web::Data<Arc<StudentService>>,
) -> Result<impl Responder, Error> {
    let id = path.into_inner();
    
    match student_service.get_student_by_id(&id).await {
        Ok(Some(student)) => Ok(HttpResponse::Ok().json(AdminResponse {
            success: true,
            message: "Student retrieved successfully".to_string(),
            data: Some(student),
        })),
        Ok(None) => Ok(HttpResponse::NotFound().json(AdminResponse::<Student> {
            success: false,
            message: "Student not found".to_string(),
            data: None,
        })),
        Err(e) => Ok(HttpResponse::InternalServerError().json(AdminResponse::<Student> {
            success: false,
            message: format!("Failed to retrieve student: {}", e),
            data: None,
        }))
    }
}

async fn create_student(
    student_dto: web::Json<CreateStudentDto>,
    student_service: web::Data<Arc<StudentService>>,
) -> Result<impl Responder, Error> {
    match student_service.create_student(student_dto.into_inner()).await {
        Ok(student) => Ok(HttpResponse::Created().json(AdminResponse {
            success: true,
            message: "Student created successfully".to_string(),
            data: Some(student),
        })),
        Err(e) => Ok(HttpResponse::BadRequest().json(AdminResponse::<Student> {
            success: false,
            message: format!("Failed to create student: {}", e),
            data: None,
        }))
    }
}

async fn update_student(
    path: web::Path<String>,
    student_dto: web::Json<UpdateStudentDto>,
    student_service: web::Data<Arc<StudentService>>,
) -> Result<impl Responder, Error> {
    let id = path.into_inner();
    
    match student_service.update_student(&id, student_dto.into_inner()).await {
        Ok(Some(student)) => Ok(HttpResponse::Ok().json(AdminResponse {
            success: true,
            message: "Student updated successfully".to_string(),
            data: Some(student),
        })),
        Ok(None) => Ok(HttpResponse::NotFound().json(AdminResponse::<Student> {
            success: false,
            message: "Student not found".to_string(),
            data: None,
        })),
        Err(e) => Ok(HttpResponse::BadRequest().json(AdminResponse::<Student> {
            success: false,
            message: format!("Failed to update student: {}", e),
            data: None,
        }))
    }
}

async fn delete_student(
    path: web::Path<String>,
    student_service: web::Data<Arc<StudentService>>,
) -> Result<impl Responder, Error> {
    let id = path.into_inner();
    
    match student_service.delete_student(&id).await {
        Ok(true) => Ok(HttpResponse::Ok().json(AdminResponse::<()> {
            success: true,
            message: "Student deleted successfully".to_string(),
            data: None,
        })),
        Ok(false) => Ok(HttpResponse::NotFound().json(AdminResponse::<()> {
            success: false,
            message: "Student not found".to_string(),
            data: None,
        })),
        Err(e) => Ok(HttpResponse::InternalServerError().json(AdminResponse::<()> {
            success: false,
            message: format!("Failed to delete student: {}", e),
            data: None,
        }))
    }
}

// === TEACHER MANAGEMENT ENDPOINTS ===

#[derive(Deserialize)]
struct TeacherQuery {
    page: Option<usize>,
    per_page: Option<usize>,
    search: Option<String>,
    department: Option<String>,
}

async fn get_all_teachers(
    query: web::Query<TeacherQuery>,
    teacher_service: web::Data<Arc<TeacherService>>,
) -> Result<impl Responder, Error> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);
    let search = query.search.clone();
    let department = query.department.clone();
    
    match teacher_service.get_all_teachers(page, per_page, search, department).await {
        Ok(teachers) => Ok(HttpResponse::Ok().json(AdminResponse {
            success: true,
            message: "Teachers retrieved successfully".to_string(),
            data: Some(teachers),
        })),
        Err(e) => Ok(HttpResponse::InternalServerError().json(AdminResponse::<Vec<Teacher>> {
            success: false,
            message: format!("Failed to retrieve teachers: {}", e),
            data: None,
        }))
    }
}

async fn get_teacher_by_id(
    path: web::Path<String>,
    teacher_service: web::Data<Arc<TeacherService>>,
) -> Result<impl Responder, Error> {
    let id = path.into_inner();
    
    match teacher_service.get_teacher_by_id(&id).await {
        Ok(Some(teacher)) => Ok(HttpResponse::Ok().json(AdminResponse {
            success: true,
            message: "Teacher retrieved successfully".to_string(),
            data: Some(teacher),
        })),
        Ok(None) => Ok(HttpResponse::NotFound().json(AdminResponse::<Teacher> {
            success: false,
            message: "Teacher not found".to_string(),
            data: None,
        })),
        Err(e) => Ok(HttpResponse::InternalServerError().json(AdminResponse::<Teacher> {
            success: false,
            message: format!("Failed to retrieve teacher: {}", e),
            data: None,
        }))
    }
}

async fn create_teacher(
    teacher_dto: web::Json<CreateTeacherDto>,
    teacher_service: web::Data<Arc<TeacherService>>,
) -> Result<impl Responder, Error> {
    match teacher_service.create_teacher(teacher_dto.into_inner()).await {
        Ok(teacher) => Ok(HttpResponse::Created().json(AdminResponse {
            success: true,
            message: "Teacher created successfully".to_string(),
            data: Some(teacher),
        })),
        Err(e) => Ok(HttpResponse::BadRequest().json(AdminResponse::<Teacher> {
            success: false,
            message: format!("Failed to create teacher: {}", e),
            data: None,
        }))
    }
}

async fn update_teacher(
    path: web::Path<String>,
    teacher_dto: web::Json<UpdateTeacherDto>,
    teacher_service: web::Data<Arc<TeacherService>>,
) -> Result<impl Responder, Error> {
    let id = path.into_inner();
    
    match teacher_service.update_teacher(&id, teacher_dto.into_inner()).await {
        Ok(Some(teacher)) => Ok(HttpResponse::Ok().json(AdminResponse {
            success: true,
            message: "Teacher updated successfully".to_string(),
            data: Some(teacher),
        })),
        })),
        Ok(None) => Ok(HttpResponse::NotFound().json(AdminResponse::<Teacher> {
            success: false,
            message: "Teacher not found".to_string(),
            data: None,
        })),
        Err(e) => Ok(HttpResponse::BadRequest().json(AdminResponse::<Teacher> {
            success: false,
            message: format!("Failed to update teacher: {}", e),
            data: None,
        }))
    }
}

async fn delete_teacher(
    path: web::Path<String>,
    teacher_service: web::Data<Arc<TeacherService>>,
) -> Result<impl Responder, Error> {
    let id = path.into_inner();
    
    match teacher_service.delete_teacher(&id).await {
        Ok(true) => Ok(HttpResponse::Ok().json(AdminResponse::<()> {
            success: true,
            message: "Teacher deleted successfully".to_string(),
            data: None,
        })),
        Ok(false) => Ok(HttpResponse::NotFound().json(AdminResponse::<()> {
            success: false,
            message: "Teacher not found".to_string(),
            data: None,
        })),
        Err(e) => Ok(HttpResponse::InternalServerError().json(AdminResponse::<()> {
            success: false,
            message: format!("Failed to delete teacher: {}", e),
            data: None,
        }))
    }
}

// === COURSE MANAGEMENT ENDPOINTS ===

#[derive(Deserialize)]
struct CourseQuery {
    page: Option<usize>,
    per_page: Option<usize>,
    search: Option<String>,
    grade_level: Option<String>,
    teacher_id: Option<String>,
    academic_year: Option<i32>,
}

async fn get_all_courses(
    query: web::Query<CourseQuery>,
    course_service: web::Data<Arc<CourseService>>,
) -> Result<impl Responder, Error> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);
    
    match course_service.get_all_courses(page as u32, per_page as u32).await {
        Ok(courses) => Ok(HttpResponse::Ok().json(AdminResponse {
            success: true,
            message: "Courses retrieved successfully".to_string(),
            data: Some(courses),
        })),
        Err(e) => Ok(HttpResponse::InternalServerError().json(AdminResponse::<Vec<Course>> {
            success: false,
            message: format!("Failed to retrieve courses: {}", e),
            data: None,
        }))
    }
}

async fn get_course_by_id(
    path: web::Path<String>,
    course_service: web::Data<Arc<CourseService>>,
) -> Result<impl Responder, Error> {
    let id = path.into_inner();
    
    // Convert string ID to UUID
    let uuid = match uuid::Uuid::parse_str(&id) {
        Ok(uuid) => uuid,
        Err(_) => return Ok(HttpResponse::BadRequest().json(AdminResponse::<Course> {
            success: false,
            message: "Invalid course ID format".to_string(),
            data: None,
        })),
    };
    
    match course_service.get_course_by_id(uuid).await {
        Ok(course) => Ok(HttpResponse::Ok().json(AdminResponse {
            success: true,
            message: "Course retrieved successfully".to_string(),
            data: Some(course),
        })),
        Err(e) => {
            if e.to_string().contains("not found") {
                Ok(HttpResponse::NotFound().json(AdminResponse::<Course> {
                    success: false,
                    message: "Course not found".to_string(),
                    data: None,
                }))
            } else {
                Ok(HttpResponse::InternalServerError().json(AdminResponse::<Course> {
                    success: false,
                    message: format!("Failed to retrieve course: {}", e),
                    data: None,
                }))
            }
        }
    }
}

async fn create_course(
    course_dto: web::Json<CreateCourseDto>,
    course_service: web::Data<Arc<CourseService>>,
) -> Result<impl Responder, Error> {
    match course_service.create_course(course_dto.into_inner()).await {
        Ok(course) => Ok(HttpResponse::Created().json(AdminResponse {
            success: true,
            message: "Course created successfully".to_string(),
            data: Some(course),
        })),
        Err(e) => Ok(HttpResponse::BadRequest().json(AdminResponse::<Course> {
            success: false,
            message: format!("Failed to create course: {}", e),
            data: None,
        }))
    }
}

async fn update_course(
    path: web::Path<String>,
    course_dto: web::Json<UpdateCourseDto>,
    course_service: web::Data<Arc<CourseService>>,
) -> Result<impl Responder, Error> {
    let id = path.into_inner();
    
    // Convert string ID to UUID
    let uuid = match uuid::Uuid::parse_str(&id) {
        Ok(uuid) => uuid,
        Err(_) => return Ok(HttpResponse::BadRequest().json(AdminResponse::<Course> {
            success: false,
            message: "Invalid course ID format".to_string(),
            data: None,
        })),
    };
    
    match course_service.update_course(uuid, course_dto.into_inner()).await {
        Ok(course) => Ok(HttpResponse::Ok().json(AdminResponse {
            success: true,
            message: "Course updated successfully".to_string(),
            data: Some(course),
        })),
        Err(e) => {
            if e.to_string().contains("not found") {
                Ok(HttpResponse::NotFound().json(AdminResponse::<Course> {
                    success: false,
                    message: "Course not found".to_string(),
                    data: None,
                }))
            } else {
                Ok(HttpResponse::BadRequest().json(AdminResponse::<Course> {
                    success: false,
                    message: format!("Failed to update course: {}", e),
                    data: None,
                }))
            }
        }
    }
}

async fn delete_course(
    path: web::Path<String>,
    course_service: web::Data<Arc<CourseService>>,
) -> Result<impl Responder, Error> {
    let id = path.into_inner();
    
    // Convert string ID to UUID
    let uuid = match uuid::Uuid::parse_str(&id) {
        Ok(uuid) => uuid,
        Err(_) => return Ok(HttpResponse::BadRequest().json(AdminResponse::<()> {
            success: false,
            message: "Invalid course ID format".to_string(),
            data: None,
        })),
    };
    
    match course_service.delete_course(uuid).await {
        Ok(_) => Ok(HttpResponse::Ok().json(AdminResponse::<()> {
            success: true,
            message: "Course deleted successfully".to_string(),
            data: None,
        })),
        Err(e) => {
            if e.to_string().contains("not found") {
                Ok(HttpResponse::NotFound().json(AdminResponse::<()> {
                    success: false,
                    message: "Course not found".to_string(),
                    data: None,
                }))
            } else {
                Ok(HttpResponse::InternalServerError().json(AdminResponse::<()> {
                    success: false,
                    message: format!("Failed to delete course: {}", e),
                    data: None,
                }))
            }
        }
    }
}

async fn assign_teacher_to_course(
    path: web::Path<(String, String)>,
    course_service: web::Data<Arc<CourseService>>,
) -> Result<impl Responder, Error> {
    let (course_id, teacher_id) = path.into_inner();
    
    // Convert string IDs to UUIDs
    let course_uuid = match uuid::Uuid::parse_str(&course_id) {
        Ok(uuid) => uuid,
        Err(_) => return Ok(HttpResponse::BadRequest().json(AdminResponse::<Course> {
            success: false,
            message: "Invalid course ID format".to_string(),
            data: None,
        })),
    };
    
    let teacher_uuid = match uuid::Uuid::parse_str(&teacher_id) {
        Ok(uuid) => uuid,
        Err(_) => return Ok(HttpResponse::BadRequest().json(AdminResponse::<Course> {
            success: false,
            message: "Invalid teacher ID format".to_string(),
            data: None,
        })),
    };
    
    match course_service.assign_teacher(course_uuid, teacher_uuid).await {
        Ok(course) => Ok(HttpResponse::Ok().json(AdminResponse {
            success: true,
            message: "Teacher assigned to course successfully".to_string(),
            data: Some(course),
        })),
        Err(e) => Ok(HttpResponse::BadRequest().json(AdminResponse::<Course> {
            success: false,
            message: format!("Failed to assign teacher to course: {}", e),
            data: None,
        }))
    }
}

async fn unassign_teacher_from_course(
    path: web::Path<String>,
    course_service: web::Data<Arc<CourseService>>,
) -> Result<impl Responder, Error> {
    let course_id = path.into_inner();
    
    // Convert string ID to UUID
    let course_uuid = match uuid::Uuid::parse_str(&course_id) {
        Ok(uuid) => uuid,
        Err(_) => return Ok(HttpResponse::BadRequest().json(AdminResponse::<Course> {
            success: false,
            message: "Invalid course ID format".to_string(),
            data: None,
        })),
    };
    
    match course_service.unassign_teacher(course_uuid).await {
        Ok(course) => Ok(HttpResponse::Ok().json(AdminResponse {
            success: true,
            message: "Teacher unassigned from course successfully".to_string(),
            data: Some(course),
        })),
        Err(e) => Ok(HttpResponse::BadRequest().json(AdminResponse::<Course> {
            success: false,
            message: format!("Failed to unassign teacher from course: {}", e),
            data: None,
        }))
    }
}

async fn get_course_stats(
    course_service: web::Data<Arc<CourseService>>,
) -> Result<impl Responder, Error> {
    // Get both grade and academic year stats
    let grade_stats_future = course_service.stats_by_grade();
    let year_stats_future = course_service.stats_by_academic_year();
    let count_future = course_service.count_courses();
    
    let (grade_stats_result, year_stats_result, count_result) = 
        futures::join!(grade_stats_future, year_stats_future, count_future);
    
    // Process results
    let grade_stats = grade_stats_result.unwrap_or_default();
    let year_stats = year_stats_result.unwrap_or_default();
    let total_count = count_result.unwrap_or(0);
    
    // Combine into a response
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "Course statistics retrieved successfully",
        "data": {
            "total_courses": total_count,
            "by_grade": grade_stats,
            "by_year": year_stats
        }
    })))
}

/// Configure all admin dashboard routes
pub fn routes() -> impl HttpServiceFactory {
    web::scope("/admin")
        // Guard all routes with AdminGuard middleware
        .guard(guard::fn_guard(move |req| AdminGuard.check(req)))
        
        // User management
        .service(
            web::scope("/users")
                .route("", web::get().to(get_all_users))
                .route("", web::post().to(create_user))
                .route("/{id}", web::get().to(get_user_by_id))
                .route("/{id}", web::put().to(update_user))
                .route("/{id}", web::delete().to(delete_user))
        )
        
        // Student management
        .service(
            web::scope("/students")
                .route("", web::get().to(get_all_students))
                .route("", web::post().to(create_student))
                .route("/{id}", web::get().to(get_student_by_id))
                .route("/{id}", web::put().to(update_student))
                .route("/{id}", web::delete().to(delete_student))
        )
        
        // Teacher management
        .service(
            web::scope("/teachers")
                .route("", web::get().to(get_all_teachers))
                .route("", web::post().to(create_teacher))
                .route("/{id}", web::get().to(get_teacher_by_id))
                .route("/{id}", web::put().to(update_teacher))
                .route("/{id}", web::delete().to(delete_teacher))
        )
        
        // Course management
        .service(
            web::scope("/courses")
                .route("", web::get().to(get_all_courses))
                .route("", web::post().to(create_course))
                .route("/{id}", web::get().to(get_course_by_id))
                .route("/{id}", web::put().to(update_course))
                .route("/{id}", web::delete().to(delete_course))
                .route("/{id}/teacher/{teacher_id}", web::put().to(assign_teacher_to_course))
                .route("/{id}/teacher", web::delete().to(unassign_teacher_from_course))
                .route("/stats", web::get().to(get_course_stats))
        )
}
