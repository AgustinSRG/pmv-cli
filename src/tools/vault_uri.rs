// Vault URI

use url::{ParseError, Url};

// URL to connect to a vault
#[derive(Clone)]
pub enum VaultURI {
    // URL to connect and login with username and password
    LoginURI {
        // Base connection URL
        base_url: Url,

        // Vault username
        username: String,

        // Vault password
        password: String,
    },

    // URL to connect to a vault with an active session
    SessionURI {
        // Base connection URL
        base_url: Url,

        // Vault active session
        session: String,
    },
}

// Error parsing a Vault URL
#[derive(Debug)]
pub enum VaultURIParseError {
    InvalidProtocol,
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

            if username.is_empty() && !pass.is_empty() {
                return Ok(VaultURI::SessionURI {
                    base_url: u,
                    session: pass,
                });
            }

            return Ok(VaultURI::LoginURI {
                base_url: u,
                password: pass,
                username,
            });
        }
        Err(e) => {
            return Err(VaultURIParseError::URLError(e));
        }
    }
}

impl VaultURI {
    pub fn to_string(&self) -> String {
        match self {
            VaultURI::LoginURI {
                base_url,
                username,
                password,
            } => {
                let mut base_url_c = base_url.clone();
                base_url_c.set_username(&username).unwrap();
                base_url_c.set_password(Some(&password)).unwrap();
                return base_url_c.to_string();
            }
            VaultURI::SessionURI { base_url, session } => {
                let mut base_url_c = base_url.clone();
                base_url_c.set_password(Some(&session)).unwrap();
                return base_url_c.to_string();
            }
        }
    }

    pub fn to_base_url(&self) -> String {
        match self {
            VaultURI::LoginURI {
                base_url,
                username: _,
                password: _,
            } => {
                return base_url.to_string();
            }
            VaultURI::SessionURI { base_url, session: _ } => {
                return base_url.to_string();
            }
        }
    }

    pub fn get_base_url(&self) -> Url {
        match self {
            VaultURI::LoginURI {
                base_url,
                username: _,
                password: _,
            } => {
                return base_url.clone();
            }
            VaultURI::SessionURI { base_url, session: _ } => {
                return base_url.clone();
            }
        }
    }

    pub fn is_login(&self) -> bool {
        match self {
            VaultURI::LoginURI {
                base_url: _,
                username: _,
                password: _,
            } => {
                return true;
            }
            VaultURI::SessionURI { base_url: _, session: _ } => {
                return false;
            }
        }
    }

    pub fn is_session(&self) -> bool {
        match self {
            VaultURI::LoginURI {
                base_url: _,
                username: _,
                password: _,
            } => {
                return false;
            }
            VaultURI::SessionURI { base_url: _, session: _ } => {
                return true;
            }
        }
    }
}
