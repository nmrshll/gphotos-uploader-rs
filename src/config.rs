// extern crate serde;
// extern crate serde_yaml;

pub use oauth2::Config as Oauth2Config;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct APIAppCredentials {
    pub client_id: String,
    pub client_secret: String,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    // #[serde(rename = "googleapi_app_credentials")]
    pub googleapi_app_credentials: APIAppCredentials,
}

pub fn load() -> Result<Config, Box<std::error::Error>> {
    let f = std::fs::File::open("config.yml")?;
    let config: Config = serde_yaml::from_reader(f)?;

    Ok(config)
}
