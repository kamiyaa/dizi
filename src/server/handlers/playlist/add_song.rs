use std::io;
use std::path::PathBuf;

use actix_web::{post, web, HttpResponse, Responder};
use serde_derive::{Deserialize, Serialize};

use crate::audio::Song;
use crate::error::AppError;
use crate::server::PLAYLIST;

#[derive(Clone, Debug, Deserialize)]
pub struct PlaylistAddSongReq {
    pub path: PathBuf,
}

#[derive(Clone, Debug, Serialize)]
pub struct PlaylistAddSongResp {
    pub song: Song,
    pub index: usize,
}
#[post("/api/playlist/add")]
pub async fn add_to_playlist(data: web::Json<PlaylistAddSongReq>) -> impl Responder {
    {
        let playlist = PLAYLIST.lock().unwrap();
        if playlist.contains(&data.path) {
            let err = io::Error::new(
                io::ErrorKind::AlreadyExists,
                "This file is already in the playlist".to_string(),
            );
            let err = AppError::from(err);
            return HttpResponse::BadRequest().json(err);
        }
    }

    match Song::new(data.path.as_path()) {
        Err(e) => HttpResponse::BadRequest().json(AppError::from(e)),
        Ok(s) => {
            let song = s.clone();
            let index = {
                let mut playlist = PLAYLIST.lock().unwrap();
                (*playlist).add_song(s);
                (*playlist).len() - 1
            };

            let res = PlaylistAddSongResp { song, index };
            HttpResponse::Ok().json(res)
        }
    }
}
