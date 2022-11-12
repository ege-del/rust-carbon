use actix_web::{post, get, web,HttpResponse, Result as AwResult};
use actix_session::{config::PersistentSession, Session, SessionMiddleware, storage::CookieSessionStore};
use maud::{html, Markup};
use crate::pages;

#[get("")]
pub async fn Index(session: Session) -> AwResult<Markup>  {
    println!("getting index call");
    Ok((pages::Index("ok")))
}

#[get("/notes")]
pub async fn notes(session: Session) -> AwResult<Markup>  {
    Ok((pages::Index("ok")))
}

#[get("/about")]
pub async fn about(session: Session) -> AwResult<Markup>  {
    Ok((pages::Index("ok")))
}

#[post("/users")]
pub async fn users(session: Session) -> AwResult<Markup>  {
    Ok((pages::Index("ok")))
}