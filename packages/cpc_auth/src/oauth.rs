use crate::error::OAuthError;

pub trait OAuthProvider: Send + Sync {
    fn get_authorization_url(&self) -> String;
    fn exchange_code(&self, code: &str) -> Result<OAuthUserInfo, OAuthError>;
}

pub struct OAuthUserInfo {
    pub email: String,
    pub name: Option<String>,
    pub provider_id: String,
}

pub struct GoogleOAuthProvider {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
}

impl GoogleOAuthProvider {
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        Self {
            client_id,
            client_secret,
            redirect_uri,
        }
    }
}

impl OAuthProvider for GoogleOAuthProvider {
    fn get_authorization_url(&self) -> String {
        format!(
            "https://accounts.google.com/o/oauth2/auth?client_id={}&redirect_uri={}&response_type=code&scope=email profile",
            self.client_id,
            self.redirect_uri
        )
    }

    fn exchange_code(&self, _code: &str) -> Result<OAuthUserInfo, OAuthError> {
        // In a real implementation, this would make an HTTP request to Google's token endpoint
        // and then fetch user info from Google's API
        Err(OAuthError::InvalidCode)
    }
}

pub struct FacebookOAuthProvider {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
}

impl FacebookOAuthProvider {
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        Self {
            client_id,
            client_secret,
            redirect_uri,
        }
    }
}

impl OAuthProvider for FacebookOAuthProvider {
    fn get_authorization_url(&self) -> String {
        format!(
            "https://www.facebook.com/v13.0/dialog/oauth?client_id={}&redirect_uri={}&scope=email",
            self.client_id,
            self.redirect_uri
        )
    }

    fn exchange_code(&self, _code: &str) -> Result<OAuthUserInfo, OAuthError> {
        // In a real implementation, this would make an HTTP request to Facebook's token endpoint
        // and then fetch user info from Facebook's API
        Err(OAuthError::InvalidCode)
    }
}