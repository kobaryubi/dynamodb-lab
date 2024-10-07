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

    async fn get_music(&self, artist: &str, song_title: &str) -> Result<Option<Music>, String> {
        let builder = self
            .client
            .get_item()
            .table_name(Self::TABLE_NAME)
            .key("artist", AttributeValue::S(artist.to_string()))
            .key("song_title", AttributeValue::S(song_title.to_string()));

        let output = builder.send().await.map_err(|e| e.to_string())?;
        if let Some(item) = output.item {
            let music = Music {
                artist: item
                    .get("artist")
                    .ok_or_else(|| "artist not found".to_string())?
                    .as_s()
                    .map_err(|_e| "can't be converted".to_string())?
                    .to_string(),
                song_title: item
                    .get("song_title")
                    .ok_or_else(|| "song_title not found".to_string())?
                    .as_s()
                    .map_err(|_e| "can't be conveted".to_string())?
                    .to_string(),
            };

            Ok(Some(music))
        } else {
            Ok(None)
        }
    }
}
