use actix_web::{get, post, HttpResponse, Responder};

#[get("/api/player")]
async fn get_player() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/api/player/play")]
async fn post_player() -> impl Responder {
    HttpResponse::Ok()
}
