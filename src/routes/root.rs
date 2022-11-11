use actix_web::{web,HttpResponse, Result as AwResult};
use actix_session::{config::PersistentSession, Session, SessionMiddleware, storage::CookieSessionStore};
use maud::{html, Markup};
use crate::pages;

#[get("")]
async fn index(session: Session) -> AwResult<Markup>  {
    Ok((pages::Index("ok")))
}

#[get("/notes")]
async fn notes(session: Session) -> AwResult<Markup>  {
    Ok((pages::Index("ok")))
}

#[get("/about")]
async fn about(session: Session) -> AwResult<Markup>  {
    Ok((pages::Index("ok")))
}

#[post("/users")]
async fn users(session: Session) -> AwResult<Markup>  {
    Ok((pages::Index("ok")))
}