use crate::service::user_service::User;
use actix_web::{delete, get, post, web, HttpResponse, Responder};

#[get("/api/user")]
async fn get_all_users(app_data: web::Data<crate::AppState>) -> impl Responder {
    let action = app_data.service_manager.user.get();
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/api/user/{id}")]
async fn get_user_id(
    app_data: web::Data<crate::AppState>,
    id: web::Path<String>,
) -> impl Responder {
    let action = app_data.service_manager.user.get_user_id(&id);
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/api/user")]
async fn add_user(app_data: web::Data<crate::AppState>, user: web::Json<User>) -> impl Responder {
    let action = app_data.service_manager.user.create(&user);
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result.inserted_id),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("api/user/update")]
async fn update_user(
    app_data: web::Data<crate::AppState>,
    user: web::Json<User>,
) -> impl Responder {
    let action = app_data.service_manager.user.update(&user);
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result.modified_count),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[delete("api/user/delete-user")]
async fn delete_user(
    app_data: web::Data<crate::AppState>,
    user: web::Json<User>,
) -> impl Responder {
    let action = app_data.service_manager.user.delete(&user.email);
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
    cfg.service(get_all_users);
    cfg.service(get_user_id);
    cfg.service(add_user);
    cfg.service(update_user);
    cfg.service(delete_user);
}
