#![allow(non_snake_case)]

#[macro_use]
extern crate serde_derive;
mod config;
mod photoslib;
mod tokensource;
use oauth2_noserver;
use tokensource::{KeyringTokenStorage, TokenSource};

use config::load as loadConfig;
// use oauth2::Config as Oauth2Config;

fn main() -> Result<(), Box<std::error::Error>> {
    let config = loadConfig()?;

    let oauthConfig = photoslib::newOauthConfig(
        config.googleapi_app_credentials.client_id,
        config.googleapi_app_credentials.client_secret,
    );
    let photosTokenStorage = KeyringTokenStorage::new("gphotos-uploader-rs", "username");
    let tokenSource = TokenSource::new(oauthConfig, Box::new(photosTokenStorage));

    // let auth_url = "https://accounts.google.com/o/oauth2/v2/auth";
    // let token_url = "https://www.googleapis.com/oauth2/v3/token";

    // // Set up the config for the Google OAuth2 process.
    // let oauthConfig = Oauth2Config::new(
    //     config.googleapi_app_credentials.client_id,
    //     config.googleapi_app_credentials.client_secret,
    //     auth_url,
    //     token_url,
    // )
    // .add_scope("https://www.googleapis.com/auth/calendar")
    // .add_scope("https://www.googleapis.com/auth/plus.me");

    // const PORT: u16 = 14565;
    // let authenticator = oauth2_noserver::Authenticator::new(oauthConfig)
    //     .set_port(PORT)
    //     .set_redirect_url(format!("http://localhost:{}/oauth/callback", PORT));
    // authenticator.authenticate().unwrap();

    let token = tokenSource.get_token().unwrap();

    Ok(())
}
