use oauth2_noserver::Authenticator as noserverAuthenticator;
use std::error::Error;

pub struct TokenSource<'a> {
    Authenticator: noserverAuthenticator,
    TokenStorage: Box<TokenStorage + 'a>,
}
impl<'a> TokenSource<'a> {
    pub fn new(
        oauthConfig: oauth2::Config,
        tokenStorage: Box<TokenStorage + 'a>,
    ) -> TokenSource<'a> {
        TokenSource {
            Authenticator: noserverAuthenticator::new(oauthConfig),
            TokenStorage: tokenStorage,
        }
    }
    pub fn get_token(self) -> Result<Token, Box<dyn Error>> {
        match self.TokenStorage.get_token() {
            Ok(token) => Ok(token),
            Err(_) => {
                // if none found, authenticate
                self.Authenticator.authenticate().expect("wot da fucc");
                Ok(String::from("sometoken"))
            }
        }
    }
}

type Token = String;

pub trait TokenStorage {
    fn get_token(self: Box<Self>) -> Result<Token, Box<dyn Error>>;
    fn set_token(&mut self, newToken: &str) -> Result<(), Box<dyn Error>>;
}

pub struct KeyringTokenStorage {
    service_name: String,
    username: String,
}
impl KeyringTokenStorage {
    pub fn new(service_name: &str, username: &str) -> Self {
        KeyringTokenStorage {
            service_name: String::from(service_name),
            username: String::from(username),
        }
    }
}
impl TokenStorage for KeyringTokenStorage {
    fn get_token(self: Box<Self>) -> Result<Token, Box<dyn Error>> {
        // try from keyring
        let keyring = keyring::Keyring::new(&self.service_name, &self.username);
        let tokenResult = keyring.get_password();

        match tokenResult {
            Ok(token) => {
                println!("The token is '{}'", token);
                Ok(token)
            }
            Err(err) => return Err(Box::new(err)),
        }
    }
    fn set_token(&mut self, newToken: &str) -> Result<(), Box<dyn Error>> {
        let keyring = keyring::Keyring::new(&self.service_name, &self.username);
        let password = "topS3cr3tP4$$w0rd";
        keyring.set_password(&password).unwrap();
        Ok(())
    }
}

// pub const photosTokenStorage: TokenStorage =
//     KeyringTokenStorage::new("my_application_name", "username");

// // let service = ;
//     let username = "username";
//     //

//     //
//
//     //
