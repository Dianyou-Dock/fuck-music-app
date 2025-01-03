use crate::application::resp::{ApplicationResp, ListResp};
use crate::types::constants::MusicSource;
use crate::types::error::ErrorHandle;
use crate::types::error::MusicClientError::NotLogin;
use crate::types::login_info::LoginInfoData;
use crate::types::play_list_info::PlayListInfoData;
use crate::types::song_info::{SongInfo, SongInfoData};
use crate::INSTANCE;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use tauri::ipc::InvokeError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LikeListReq {
    pub source: MusicSource,
    pub offset: usize,
    pub limit: usize,
}

#[tauri::command]
pub async fn like_list(req: LikeListReq) -> Result<ApplicationResp<ListResp>, InvokeError> {
    if INSTANCE.read().await.netesae.login_info().is_none() {
        return Err(InvokeError::from_anyhow(NotLogin.anyhow_err()));
    };

    let mut instance = INSTANCE.write().await;

    let skip = req.offset * req.limit;

    let resp = match req.source {
        MusicSource::Netesae => {


            let list_info = instance.netesae.like_list().unwrap();
            let data = match &list_info.data {
                PlayListInfoData::Netesae(v) => v,
            };

            let take = if req.limit == 0 {
                data.songs.len()
            } else {
                req.limit
            };

            let total = data.songs.len();
            let page_list = data
                .songs
                .iter()
                .skip(skip)
                .take(take)
                .map(|v| SongInfo {
                    data: SongInfoData::Netesae(v.clone()),
                })
                .collect::<Vec<_>>();
            ListResp {
                id: data.id,
                name: data.name.clone(),
                cover_img_url: data.cover_img_url.clone(),
                songs: page_list,
                likeds: vec![],
                total: total as u64,
            }
        }
        MusicSource::Spotify => {
            todo!()
        }
        MusicSource::QQ => {
            todo!()
        }
        MusicSource::Apple => {
            todo!()
        }
    };

    Ok(ApplicationResp::success_data(resp))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LikeSongReq {
    pub source: MusicSource,
    pub song_id: u64,
    pub is_like: bool,
}

#[tauri::command]
pub async fn like_song(req: LikeSongReq) -> Result<ApplicationResp<bool>, InvokeError> {
    let Some(login_info) = INSTANCE.read().await.netesae.login_info() else {
        return Err(InvokeError::from_anyhow(NotLogin.anyhow_err()));
    };

    let mut instance = INSTANCE.write().await;

    match req.source {
        MusicSource::Netesae => {
            let result = instance
                .netesae
                .client()
                .like_song(req.song_id, req.is_like)
                .await
                .map_err(InvokeError::from_anyhow)?;

            // update like list
            let user_info = match login_info.data {
                LoginInfoData::Netesae(v) => v,
            };
            {
                let like_list = instance
                    .netesae
                    .client()
                    .like_list(user_info.uid)
                    .await
                    .map_err(InvokeError::from_anyhow)?;
                instance
                    .netesae
                    .set_like_list(like_list)
                    .map_err(InvokeError::from_anyhow)?;
            }
            Ok(ApplicationResp::success_data(result))
        }
        MusicSource::Spotify => {
            todo!()
        }
        MusicSource::QQ => {
            todo!()
        }
        MusicSource::Apple => {
            todo!()
        }
    }
}
