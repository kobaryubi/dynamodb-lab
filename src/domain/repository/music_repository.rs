use crate::domain::entity::music::Music;

#[trait_variant::make(MusicRepository: Send)]
pub trait LocalMusicRepository {
    async fn add_music(&self, music: Music) -> Result<(), String>;
}
