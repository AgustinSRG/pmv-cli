// Vault URI

use url::{Url, ParseError};

// URL to connect and login with username and password
#[derive(Clone)]
pub struct VaultLoginURI {
    // Base connection URL
    pub base_url: Url,

    // Vault username
    pub username: String,

    // Vault password
    pub password: String,
}


// URL to connect to a vault with an active session
#[derive(Clone)]
pub struct VaultSessionURI {
    // Base connection URL
    pub base_url: Url,

    // Vault active session
    pub session: String,
}

// URL to connect to a vault
#[derive(Clone)]
pub enum VaultURI {
    LoginURI(VaultLoginURI),
    SessionURI(VaultSessionURI),
}

// Error parsing a Vault URL
pub enum VaultURIParseError {
    InvalidProtocol,
    NoCredentialsSpecified,
    URLError(ParseError),
}

// Parses a vault URL
pub fn parse_vault_uri(uri: String) -> Result<VaultURI, VaultURIParseError> {
    let r = Url::parse(&uri);

    match r {
        Ok(mut u) => {
            if u.scheme() != "http" && u.scheme() != "https" {
                return Err(VaultURIParseError::InvalidProtocol);
            }

            if u.cannot_be_a_base() || !u.has_host() {
                return Err(VaultURIParseError::InvalidProtocol);
            }

            let username = u.username().to_string();
            let pass = u.password().unwrap_or("").to_string();

            u.set_username("").unwrap();
            u.set_password(None).unwrap();

            if pass.chars().count() == 0 {
                return Err(VaultURIParseError::NoCredentialsSpecified);
            }

            if username.chars().count() == 0 {
                return Ok(VaultURI::SessionURI(VaultSessionURI{
                    base_url: u,
                    session: pass,
                }))
            }

            return Ok(VaultURI::LoginURI(VaultLoginURI{
                base_url: u,
                password: pass,
                username,
            }))
        },
        Err(e) =>  {
            return Err(VaultURIParseError::URLError(e));
        },
    }
}

impl VaultURI {
    fn to_string(&self) -> String {
        match self {
            VaultURI::LoginURI(u) => {
                let mut base_url = u.base_url.clone();
                base_url.set_username(&u.username).unwrap();
                base_url.set_password(Some(&u.password)).unwrap();
                return base_url.to_string();
            },
            VaultURI::SessionURI(u) => {
                let mut base_url = u.base_url.clone();
                base_url.set_password(Some(&u.session)).unwrap();
                return base_url.to_string();
            },
        }
    }
}
