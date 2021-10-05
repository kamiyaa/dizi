use actix_web::{get, HttpResponse, Responder};

use crate::server::PLAYLIST;

#[get("/api/playlist")]
pub async fn get_playlist() -> impl Responder {
    let playlist = PLAYLIST.lock().unwrap();
    let playlist_list = (*playlist).playlist().clone();
    HttpResponse::Ok().json(playlist_list)
}
