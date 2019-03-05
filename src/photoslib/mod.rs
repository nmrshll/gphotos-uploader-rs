use oauth2;
// mod hub;
mod scope;

pub const API_VERSION: &str = "v1";
pub const API_BASEPATH: &str = "https://photoslibrary.googleapis.com/";

pub const AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
pub const TOKEN_URL: &str = "https://www.googleapis.com/oauth2/v3/token";

pub fn newOauthConfig(client_id: String, client_secret: String) -> oauth2::Config {
    return oauth2::Config::new(client_id, client_secret, AUTH_URL, TOKEN_URL)
        .add_scope("https://www.googleapis.com/auth/calendar")
        .add_scope("https://www.googleapis.com/auth/plus.me");
}
