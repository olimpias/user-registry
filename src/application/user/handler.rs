use uuid::Uuid;
use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use tracing::Instrument;
use crate::domain::user::entity;
use crate::domain::user::service;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DtoUser {
    name: String,
    id: String
}

#[derive(Deserialize)]
pub struct Info {
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListUserResponse {
    users: Vec<DtoUser>
}

fn is_valid_name(name: &str) -> bool {
    let is_empty_or_whitespace = name.trim().is_empty();

    let is_too_long = UnicodeSegmentation::graphemes(name, true).count() > 256;

    let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\','{','}'];

    let contains_forbidden_chars = name.chars().any(|a| forbidden_chars.contains(&a));

    !(is_empty_or_whitespace || is_too_long || contains_forbidden_chars)
}

#[tracing::instrument(
    name = "creating a new user",
    skip(service, item, _req),
    fields(
        request_id = %Uuid::new_v4(),
        username = %item.0.name
    )
)]
pub async fn create_user(service: web::Data<service::Service>, item: web::Json<CreateUserRequest>, _req: HttpRequest) -> HttpResponse {
    if !is_valid_name(&item.0.name) {
        return HttpResponse::BadRequest().finish();
    }
    let user = entity::User{name: item.0.name, id: Uuid::nil()};
    match service.create_user(user).await {
        Ok(res) => HttpResponse::Created().json(DtoUser{id: res.id.to_string(), name: res.name}),
        Err(err) => {
            tracing::error!("unable to create user {}", err);
            HttpResponse::InternalServerError().into()
            }
    }
}

pub async fn list_users(service: web::Data<service::Service>, _req: HttpRequest) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!("listing all users",%request_id);
    let _request_span_guard = request_span.enter();

    match service.get_users().await {
        Ok(res) => {
            let mut result = Vec::new();
            for user in res {
                result.push(DtoUser{id: user.id.to_string(), name: user.name});
            }
            HttpResponse::Ok().json(ListUserResponse{users: result})
        }
        Err(err) => {
            tracing::error!("unable to create user {}", err);
            HttpResponse::InternalServerError().into()
            }
    }
}

pub async fn get_user(service: web::Data<service::Service>, info: web::Path<Info>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!("getting user",%request_id, userId=%info.id);
    let _request_span_guard = request_span.enter();
    
    let user_id = match Uuid::parse_str(info.id.as_str()) {
        Ok(id)=> id,
        Err(err)=> {
            tracing::error!("invalid uuid {}", err);
            return HttpResponse::BadRequest().into()
        }
    };

    let operation_span = tracing::info_span!("getting a user into storage"); 
    match service.get_user(user_id).instrument(operation_span).await {
        Ok(res) => HttpResponse::Ok().json(DtoUser{id: res.id.to_string(), name: res.name}),
        Err(err) => {
            tracing::error!("unable to create user {}", err);
            HttpResponse::InternalServerError().into()
            }
    }
}

pub async fn delete_user(service: web::Data<service::Service>, info: web::Path<Info>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!("deleting user",%request_id, userId=%info.id);
    let _request_span_guard = request_span.enter();

    let user_id = match Uuid::parse_str(info.id.as_str()) {
        Ok(id)=> id,
        Err(err)=> {
            tracing::error!("invalid uuid {}", err);
            return HttpResponse::BadRequest().into()
        }
    };
    match service.delete_user(user_id).await {
        Ok(_) => {
            HttpResponse::Accepted().into()
        }
        Err(err) => {
            tracing::error!("unable to create user {}", err);
            HttpResponse::InternalServerError().into()
            }
    }
}

