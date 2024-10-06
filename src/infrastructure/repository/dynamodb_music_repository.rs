use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;

use crate::domain::{entity::music::Music, repository::music_repository::MusicRepository};

pub struct DynamoDBMusicRepository {
    client: Client,
}

impl DynamoDBMusicRepository {
    const TABLE_NAME: &str = "music";

    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

impl MusicRepository for DynamoDBMusicRepository {
    async fn add_music(&self, music: Music) -> Result<(), String> {
        let builder = self
            .client
            .put_item()
            .table_name(Self::TABLE_NAME)
            .item("artist", AttributeValue::S(music.artist))
            .item("song_title", AttributeValue::S(music.song_title));

        builder.send().await.map_err(|e| e.to_string())?;

        Ok(())
    }
}
