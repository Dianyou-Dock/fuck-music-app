use crate::application::resp::ApplicationResp;
use crate::application::Source;
use crate::types::play_list_info::PlayListInfoData;
use crate::types::song_info::{SongInfo, SongInfoData};
use crate::INSTANCE;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use tauri::ipc::InvokeError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LikeListReq {
    pub source: Source,
    pub user_id: u64,
    pub offset: usize,
    pub limit: usize,
}

#[derive(Serialize, Debug, Clone)]
pub struct LikeListResp<T: Serialize + Clone + Debug> {
    pub song_infos: Vec<T>,
}

#[tauri::command]
pub async fn like_list(
    req: LikeListReq,
) -> Result<ApplicationResp<LikeListResp<SongInfo>>, InvokeError> {
    let mut instance = INSTANCE.write().await;

    let offset = req.offset * req.limit;
    let limit = req.limit;

    let list = match req.source {
        Source::Netesae => {
            let empty = instance.netesae.like_list().is_none();

            if empty {
                let like_list = instance
                    .netesae
                    .client()
                    .like_list(req.user_id)
                    .await
                    .map_err(InvokeError::from_anyhow)?;
                instance
                    .netesae
                    .set_like_list(like_list)
                    .map_err(InvokeError::from_anyhow)?;
            }

            let info = instance.netesae.like_list().unwrap();
            let data = match &info.data {
                PlayListInfoData::Netesae(v) => v,
            };
            let page_list = data
                .songs
                .iter()
                .skip(offset)
                .take(limit)
                .map(|v| SongInfo {
                    data: SongInfoData::Netesae(v.clone()),
                })
                .collect::<Vec<_>>();
            page_list
        }
        Source::Spotify => {
            todo!()
        }
        Source::QQ => {
            todo!()
        }
        Source::Apple => {
            todo!()
        }
    };

    Ok(ApplicationResp::success_data(LikeListResp {
        song_infos: list,
    }))
}
