use crate::domain::entity::music::Music;

#[trait_variant::make(MusicRepository: Send)]
pub trait LocalMusicRepository {
    async fn add_music(&self, music: Music) -> Result<(), String>;
    async fn get_music(&self, artist: &str, song_title: &str) -> Result<Option<Music>, String>;
}
