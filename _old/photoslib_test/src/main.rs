extern crate hyper;
extern crate hyper_rustls;
extern crate yup_oauth2 as oauth2;
extern crate google_photoslibrary1 as photoslibrary1;
use photoslibrary1::{Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use photoslibrary1::PhotosLibrary;

fn main() -> Result<(), Box<std::error::Error>> {
    // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
    // `client_secret`, among other things. 
    let secret= ApplicationSecret {
        client_id: String::from("20637643488-1hvg8ev08r4tc16ca7j9oj3686lcf0el.apps.googleusercontent.com"),
        client_secret: String::from("0JyfLYw0kyDcJO-pGg5-rW_P"),
        ..Default::default()
    };
    // dbg!("{}",secret);
    
    // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
    // unless you replace  `None` with the desired Flow.
    // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
    // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
    // retrieve them from storage.
    let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                                hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
                                <MemoryStorage as Default>::default(), None);
    let mut hub = PhotosLibrary::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
    // You can configure optional parameters by calling the respective setters at will, and
    // execute the final call using `doit()`.
    // Values shown here are possibly random and not representative !
    let result = hub.albums().list()
                .page_token("sed")
                .page_size(-85)
                .exclude_non_app_created_data(true)
                .doit();
    
    match result {
        Err(e) => match e {
            // The Error enum provides details about what exactly happened.
            // You can also just use its `Debug`, `Display` or `Error` traits
            Error::HttpError(_)
            |Error::MissingAPIKey
            |Error::MissingToken(_)
            |Error::Cancelled
            |Error::UploadSizeLimitExceeded(_, _)
            |Error::Failure(_)
            |Error::BadRequest(_)
            |Error::FieldClash(_)
            |Error::JsonDecodeError(_, _) => println!("{}", e),
        },
        Ok(res) => println!("Success: {:?}", res),
    };

    Ok(())
}
 
