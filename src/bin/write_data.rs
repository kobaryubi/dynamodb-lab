use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::{config, Client};
use dynamodb_lab::domain::entity::music::Music;
use dynamodb_lab::domain::repository::music_repository::MusicRepository;
use dynamodb_lab::dynamodb::{DYNAMODB_ENDPOINT_URL, REGION};
use dynamodb_lab::infrastructure::repository::dynamodb_music_repository::DynamoDBMusicRepository;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let region_provider = RegionProviderChain::default_provider().or_else(REGION);
    let sdk_config = aws_config::defaults(BehaviorVersion::latest())
        .test_credentials()
        .endpoint_url(DYNAMODB_ENDPOINT_URL)
        .region(region_provider)
        .load()
        .await;
    let config = config::Builder::from(&sdk_config).build();
    let client = Client::from_conf(config);

    let dynamodb_music_repository = DynamoDBMusicRepository::new(client);
    dynamodb_music_repository
        .add_music(Music {
            artist: format!("No One You"),
            song_title: format!("Call Me Today"),
        })
        .await?;

    Ok(())
}
