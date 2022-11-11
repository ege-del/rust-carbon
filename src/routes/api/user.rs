use actix_web::{web,HttpResponse};
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};
mod model;

impl User{
    #[post("/register")]
    pub async fn register(client: web::Data<Client>, form: web::Form<model::User>) -> HttpResponse {
        let collection = client.database(DB_NAME).collection(COLL_NAME);
        let result = collection.insert_one(form.into_inner(), None).await;
        match result {
            Ok(_) => HttpResponse::Ok().body("user added"),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
    }


    #[post("/login")]
    pub async  fn login(client: web::Data<Client>, form: web::Form<model::User>) -> HttpResponse {
        let collection = client.database(DB_NAME).collection(COLL_NAME);
        let result = collection.insert_one(form.into_inner(), None).await;
        match result {
            Ok(_) => HttpResponse::Ok().body("user added"),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
    }


    #[post("/logout")]
    pub async  fn logout(client: web::Data<Client>, form: web::Form<model::User>) -> HttpResponse {
        let collection = client.database(DB_NAME).collection(COLL_NAME);
        let result = collection.insert_one(form.into_inner(), None).await;
        match result {
            Ok(_) => HttpResponse::Ok().body("user added"),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
    }
}