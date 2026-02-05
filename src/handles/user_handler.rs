use actix_web::{web, HttpResponse, Responder};
use actix_web::web::Json;
use firebase_auth_sdk::FireAuth;
use crate::models::user::{CreateUser};

pub async fn sign_up(
    service: web::Data<FireAuth>,
    user: Json<CreateUser>
) -> impl Responder {
    let result = service.sign_up_email(
        &user.email.as_str(),
        &user.password.as_str(),
        true
    ).await;

    match result {
        Ok(res) => {
            HttpResponse::Ok().json(res)
        }
        Err(e) => {
            eprintln!("Erro ao criar usuário: {:?}", e);
            HttpResponse::BadRequest().body(format!("Erro no cadastro: {}", e))
        }
    }
}

pub async fn login(
    service: web::Data<FireAuth>,
    user: Json<CreateUser>
) -> impl Responder {
    let result = service.sign_in_email(
        &user.email.as_str(),
        &user.password.as_str(),
        true
    ).await;

    match result {
        Ok(response) => {
            HttpResponse::Created().json(response)
        }
        Err(e) => {
            eprintln!("Erro ao logar usuário: {:?}", e);
            HttpResponse::BadRequest().body(format!("Erro no cadastro: {}", e))
        }
    }
}