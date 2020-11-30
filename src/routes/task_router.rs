use crate::service::task_service::Task;
use actix_web::{delete, get, post, web, HttpResponse, Responder};

#[get("/api/task")]
async fn get_all_tasks(app_data: web::Data<crate::AppState>) -> impl Responder {
    let action = app_data.service_manager.task.get();
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/api/task/{id}")]
async fn get_task_id(
    app_data: web::Data<crate::AppState>,
    id: web::Path<String>,
) -> impl Responder {
    let action = app_data.service_manager.task.get_task_id(&id);
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/api/task")]
async fn add_task(app_data: web::Data<crate::AppState>, task: web::Json<Task>) -> impl Responder {
    let action = app_data.service_manager.task.create(&task);
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result.inserted_id),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("api/task/{id}")]
async fn update_task(
    app_data: web::Data<crate::AppState>,
    task: web::Json<Task>,
) -> impl Responder {
    let action = app_data.service_manager.task.update(&task);
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result.modified_count),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[delete("api/task/{id}")]
async fn delete_task(
    app_data: web::Data<crate::AppState>,
    id: web::Path<String>,
) -> impl Responder {
    let action = app_data.service_manager.task.delete(&id);
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result.deleted_count),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_tasks);
    cfg.service(get_task_id);
    cfg.service(add_task);
    cfg.service(update_task);
    cfg.service(delete_task);
}
