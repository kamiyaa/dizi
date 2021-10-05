use std::io;

use actix_web::{post, web, HttpResponse, Responder};
use serde_derive::{Deserialize, Serialize};

use crate::audio::Song;
use crate::error::AppError;
use crate::server::PLAYLIST;

#[derive(Clone, Debug, Deserialize)]
pub struct PlaylistRemoveSongReq {
    pub index: usize,
}
#[derive(Clone, Debug, Serialize)]
pub struct PlaylistRemoveSongResp {
    pub index: usize,
    pub song: Song,
}
#[post("/api/playlist/remove")]
pub async fn remove_from_playlist(data: web::Json<PlaylistRemoveSongReq>) -> impl Responder {
    let mut playlist = PLAYLIST.lock().unwrap();
    if data.index >= playlist.len() {
        let err = io::Error::new(
            io::ErrorKind::InvalidInput,
            "Index out of bounds".to_string(),
        );
        let err = AppError::from(err);
        return HttpResponse::BadRequest().json(err);
    }

    let song = (*playlist).remove_song(data.index);

    let res = PlaylistRemoveSongResp {
        index: data.index,
        song,
    };
    HttpResponse::Ok().json(res)
}
