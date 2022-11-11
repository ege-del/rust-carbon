use actix_web::{web,HttpResponse};
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};
mod model;

impl Note{
    #[post("/add")]
    pub async  fn add(client: web::Data<Client>, form: web::Form<model::User>) -> HttpResponse {
        let collection = client.database(DB_NAME).collection(COLL_NAME);
        let result = collection.insert_one(form.into_inner(), None).await;
        match result {
            Ok(_) => HttpResponse::Ok().body("user added"),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
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