use actix_web::{post, get, web, HttpResponse};
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};
use crate::model;


#[post("/register")]
pub async fn register(client: web::Data<Client>, form: web::Form<model::User>) -> HttpResponse {
    let collection = client.database(dotenv!("MONGO_DB")).collection("user");
    let result = collection.insert_one(form.into_inner(), None).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("user added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}


#[post("/login")]
pub async  fn login(client: web::Data<Client>, form: web::Form<model::User>) -> HttpResponse {
    let collection = client.database(dotenv!("MONGO_DB")).collection("user");
    let result = collection.insert_one(form.into_inner(), None).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("user added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}


#[post("/logout")]
pub async  fn logout(client: web::Data<Client>, form: web::Form<model::User>) -> HttpResponse {
    let collection = client.database(dotenv!("MONGO_DB")).collection("user");
    let result = collection.insert_one(form.into_inner(), None).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("user added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}