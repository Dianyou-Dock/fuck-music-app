pub mod impls;

use crate::types::login_info::{LoginInfo, LoginQrInfo};
use crate::types::play_list_info::{PlayListInfo, SongListInfo};
use crate::types::song_info::SongInfo;
use crate::types::song_url::{SongRate, SongUrl};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Client: Sync + Send {
    async fn login_qr(&mut self) -> Result<LoginQrInfo>;

    async fn login_by_unikey(&mut self, unikey: String) -> Result<(i32, Option<LoginInfo>)>;

    async fn logout(&mut self) -> Result<()>;

    async fn like_list(&mut self, user_id: u64) -> Result<PlayListInfo>;

    async fn collect_list(&mut self, user_id: u64) -> Result<Vec<SongListInfo>>;

    async fn list_detail(&mut self, list_id: u64) -> Result<PlayListInfo>;

    async fn song_infos(&mut self, song_id_list: &[u64]) -> Result<Vec<SongInfo>>;

    async fn search_song(&mut self, song: &str, singer: &str) -> Result<Option<SongInfo>>;

    async fn like_song(&mut self, song_id: u64, is_like: bool) -> Result<bool>;

    async fn songs_url(&mut self, songs: &[u64], song_rate: SongRate) -> Result<Vec<SongUrl>>;

    async fn logged(&mut self) -> bool;

    fn set_logged(&mut self, logged: bool);

    async fn login_info(&mut self) -> Result<LoginInfo>;
}
