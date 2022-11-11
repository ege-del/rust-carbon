#[cfg(test)]
mod test;

use actix_web::{middleware::Logger, guard, get, Error, post,web, App, cookie::{self, Key}, HttpResponse, HttpServer, Responder, Result as AwResult};
use actix_files as fs;
// use env_logger::Env;
use env_logger::{Env, Builder, WriteStyle};
use actix_session::{config::PersistentSession, Session, SessionMiddleware, storage::CookieSessionStore};

// not used right now, trying to figure out custom builder
// use log::{Level, LevelFilter, Log, MetadataBuilder, Record};


// API
mod routes;
use routes::api;

// MIDDLEWARES
mod middleware;

// TEMPLATES
use maud::{html, Markup};
mod pages;

// DATABASE
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};
mod model;
use model::User;

const DB_NAME: &str = "myApp";
const COLL_NAME: &str = "users";

/// Adds a new user to the "users" collection in the database.
#[post("/add")]
async fn add_user(client: web::Data<Client>, form: web::Form<User>) -> HttpResponse {
    let collection = client.database(DB_NAME).collection(COLL_NAME);
    let result = collection.insert_one(form.into_inner(), None).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("user added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

/// Gets the user with the supplied username.
#[get("/{username}")]
async fn get_user(client: web::Data<Client>, username: web::Path<String>) -> HttpResponse {
    let username = username.into_inner();
    let collection: Collection<User> = client.database(DB_NAME).collection(COLL_NAME);
    match collection
        .find_one(doc! { "username": &username }, None)
        .await
    {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => {
            HttpResponse::NotFound().body(format!("No user found with username {username}"))
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}


#[get("/lvl2")]
async fn guard_2(session: Session) -> AwResult<Markup>  {
    println!("guarded page is called");
    Ok((pages::Index("ok")))
}

#[get("/")]
async fn guarded_page(session: Session) -> AwResult<Markup>  {
    println!("guarded page is called");
    Ok((pages::Index("ok")))
}

#[get("/")]
async fn index(session: Session) -> AwResult<Markup>  {
    if let Some(count) = session.get::<i32>("counter")? {
        session.insert("counter", count + 1)?;
    } else {
        session.insert("counter", 1)?;
    }
    // Ok(HttpResponse::Ok().body(format!(
    //     "Count is {:?}!",
    //     session.get::<i32>("counter")?.unwrap()
    // )))

    // HttpResponse::Ok().body(
    //     format!("{}",
    //     pages::index::Page {
    //         title: "Example Domain"
    //     })
    // )
    Ok((pages::Index("ok")))
}



/// Creates an index on the "username" field to force the values to be unique.
async fn create_username_index(client: &Client) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! { "username": 1 })
        .options(options)
        .build();
    
    client.database(DB_NAME)
        .collection::<User>(COLL_NAME)
        .create_index(model, None)
        .await
        .expect("creating an index should succeed");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());

    let client = Client::with_uri_str(uri).await.expect("failed to connect");
    create_username_index(&client).await;

    // let error_metadata = MetadataBuilder::new()
    //     .target("myApp")
    //     .level(Level::Error)
    //     .build();

    // let built = Record::builder()
    //     .metadata(error_metadata)
    //     .args(format_args!("Error!"))
    //     .line(Some(433))
    //     .file(Some("app.rs"))
    //     .module_path(Some("server"))
    //     .build();

    // let stylish_logger = Builder::new()
    //     .filter(None, LevelFilter::Error)
    //     .write_style(WriteStyle::Always)
    //     .build();


    // env_logger::init_from_env(stylish_logger);
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    
    // .wrap(Logger::new("%a %{User-Agent}i"))

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(false)
                    .build()
            )
            .wrap(middleware::SayHi)
            // what this ?  .app_data(web::Data::new(client.clone()))
            .app_data(web::Data::new(client.clone()))
            .service(fs::Files::new("/public", "./src/public").show_files_listing())
            .service(
                web::scope("/user")
                    .guard(guard::Get())
                    .service(add_user)
                    .service(get_user),
            ).service(
                web::scope("/guarded_page")
                    .guard(guard::Get())
                    .service(guarded_page)
            ).service(
                web::scope("")
                    .service(index)
            ).service(
                web::scope("api/user")
                    .service(api::register)
                    .service(api::login)
                    .service(api::logout)
            ).service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}