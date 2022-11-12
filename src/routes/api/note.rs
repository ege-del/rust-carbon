use actix_web::{post, get, web,Responder, Result, HttpResponse};
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};
use crate::model;
use serde::Serialize;

#[derive(Serialize)]
struct MyObj {
    name: String,
}

#[get("/path")]
pub async fn path() -> HttpResponse {
    HttpResponse::Ok().body("data")
}

#[post("/add")]
pub async  fn add(client: web::Data<Client>, form: web::Json<model::Note>) -> Result<impl Responder> {
    let erre = MyObj {
        name: "err".to_string(),
    };
    let oke = MyObj {
        name: "ok".to_string(),
    };
    println!("adding to {}",dotenv!("MONGO_DB"));
    let collection = client.database(dotenv!("MONGO_DB")).collection("notes");
    let result = collection.insert_one(form, None).await;

    match result {
        Ok(_) => Ok(web::Json(oke)),
        Err(err) => Ok(web::Json(erre)),
    }
}
// #[get("/get")]
// async fn get(session: Session) -> AwResult<Markup>  {
    
// }

// #[get("/add")]
// async fn add(session: Session) -> AwResult<Markup>  {
    
// }

// #[get("/delete")]
// async fn delete(session: Session) -> AwResult<Markup>  {
    
// }