use aws_config::{meta::region::RegionProviderChain, BehaviorVersion};
use aws_sdk_dynamodb::{
    config,
    types::{
        AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType,
    },
    Client, Error,
};
use dynamodb_lab::dynamodb::{DYNAMODB_ENDPOINT_URL, REGION};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let region_provider = RegionProviderChain::default_provider().or_else(REGION);
    let sdk_config = aws_config::defaults(BehaviorVersion::latest())
        .test_credentials()
        .endpoint_url(DYNAMODB_ENDPOINT_URL)
        .region(region_provider)
        .load()
        .await;
    let config = config::Builder::from(&sdk_config).build();
    let client = Client::from_conf(config);

    let table_name = "Music";
    let partition_key = KeySchemaElement::builder()
        .attribute_name("Artist")
        .key_type(KeyType::Hash)
        .build()?;
    let sort_key = KeySchemaElement::builder()
        .attribute_name("SongTitle")
        .key_type(KeyType::Range)
        .build()?;
    let artist_attribute = AttributeDefinition::builder()
        .attribute_name("Artist")
        .attribute_type(ScalarAttributeType::S)
        .build()?;
    let song_title_attribute = AttributeDefinition::builder()
        .attribute_name("SongTitle")
        .attribute_type(ScalarAttributeType::S)
        .build()?;
    let provisioned_throughput = ProvisionedThroughput::builder()
        .read_capacity_units(1)
        .write_capacity_units(1)
        .build()?;

    let output = client
        .create_table()
        .table_name(table_name)
        .key_schema(partition_key)
        .key_schema(sort_key)
        .attribute_definitions(artist_attribute)
        .attribute_definitions(song_title_attribute)
        .provisioned_throughput(provisioned_throughput)
        .send()
        .await;

    match output {
        Ok(output) => {
            println!("Created table: {:#?}", output);
            Ok(())
        }
        Err(e) => {
            eprintln!("{}", e);
            Err(Error::from(e))
        }
    }
}
