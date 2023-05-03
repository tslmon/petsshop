use merge::Merge;
use serde::Deserialize;
use std::net::IpAddr;

#[derive(Debug, Deserialize, Clone, Merge)]
pub struct Settings {
    pub(crate) database: Option<DatabaseConfig>,
    pub(crate) rate_limit: Option<RateLimitConfig>,
    pub(crate) hostname: Option<String>,
    pub(crate) bind: Option<IpAddr>,
    pub(crate) port: Option<u16>,
    pub(crate) multi_instance_mode: Option<bool>,
    pub(crate) tls_enabled: Option<bool>,
    pub(crate) jwt_secret: Option<String>,
    pub(crate) files_url: Option<String>,
    pub(crate) iframely_url: Option<String>,
    pub(crate) captcha: Option<CaptchaConfig>,
    pub(crate) email: Option<EmailConfig>,
    pub(crate) setup: Option<SetupConfig>,
    pub(crate) session_key: Option<[u8; 32]>,
    pub(crate) social_provider: Option<SocialProviderConfig>,
    pub(crate) social_provider_url: Option<SocialProviderUrl>,
    pub(crate) access_token: Option<TokenConfig>,
    pub(crate) refresh_token: Option<TokenConfig>,
    pub(crate) id_token: Option<TokenConfig>,
    pub(crate) state_token: Option<TokenConfig>,
    pub(crate) password_policy: Option<PasswordConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub enum PasswordRecoveryMode {
    QA,
    SMS,
    EMAIL,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PasswordRecovery {
    pub mode: PasswordRecoveryMode,
    pub failed_login_attemt: Option<u16>,
    pub password_lockout_time: Option<u16>,
    pub has_expire: bool,
    pub grace_login_count: Option<u16>,
    pub expire_time: Option<u16>,
    pub has_recover: bool,
    pub recovery_op: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PasswordConfig {
    pub user_lock: bool,
    pub failed_login_attemt: Option<u16>,
    pub password_lockout_time: Option<u16>,
    pub has_expire: bool,
    pub grace_login_count: Option<u16>,
    pub expire_time: Option<u16>,
    pub has_recover: bool,
    pub recovery_op: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub enum TokenPrefix {
    AT,
    RT,
    ST,
    IT,
}

impl From<String> for TokenPrefix {
    fn from(_v: String) -> Self {
        Self::RT
    }
}

// #[derive(Debug, PartialEq, Deserialize, Clone)]
// pub enum Algorithm {
//     HMAC256,
//     HMAC512,
//     RSA256,
//     RSA512,
// }

#[derive(Debug, Deserialize, Clone)]
pub struct TokenAlgorithm {
    pub algorithm: String,
    pub public_key: Option<String>,
    pub private_key: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TokenConfig {
    pub prefix: TokenPrefix,
    pub validation: Validation,
    pub algorithm: TokenAlgorithm,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Validation {
    pub leeway: u64,
    pub validate_exp: bool,
    pub validate_nbf: bool,
    pub aud: Option<Vec<String>>,
    pub iss: Option<String>,
    pub sub: Option<String>,
    pub algorithm: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CaptchaConfig {
    pub enabled: bool,
    pub difficulty: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: i32,
    pub database: String,
    pub pool_size: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EmailConfig {
    pub smtp_server: String,
    pub smtp_login: Option<String>,
    pub smtp_password: Option<String>,
    pub smtp_from_address: String,
    pub use_tls: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RateLimitConfig {
    pub signups: i32,
    pub signups_per_second: i32,
    pub signins: i32,
    pub signins_per_second: i32,
    pub invites: i32,
    pub invites_per_second: i32,
    pub settings: i32,
    pub settings_per_second: i32,
    pub recoveries: i32,
    pub recoveries_per_second: i32,
    pub verify: i32,
    pub verify_per_second: i32,
    pub otp: i32,
    pub otp_per_second: i32,
    pub token: i32,
    pub token_per_second: i32,
    pub user: i32,
    pub user_per_second: i32,
    pub signout: i32,
    pub signout_per_second: i32,
    pub authorize: i32,
    pub authorize_per_second: i32,
    pub callback: i32,
    pub callback_per_second: i32,
    pub audit: i32,
    pub audit_per_second: i32,
    pub users: i32,
    pub users_per_second: i32,
    pub generate_link: i32,
    pub generate_link_per_second: i32,
    pub saml: i32,
    pub saml_per_second: i32,
    pub tenants: i32,
    pub tenants_per_second: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SetupConfig {
    pub admin_username: String,
    pub admin_password: String,
    pub admin_email: Option<String>,
    pub instance_name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SocialProviderConfig {
    pub github_enabled: Option<bool>,
    pub github_client_id: String,
    pub github_client_secret: String,
    pub google_enabled: Option<bool>,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub fb_enabled: Option<bool>,
    pub fb_client_id: String,
    pub fb_client_secret: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SocialProviderUrl {
    pub github_auth_url: String,
    pub github_token_url: String,
    pub google_auth_url: String,
    pub google_token_url: String,
    pub fb_auth_url: String,
    pub fb_token_url: String,
}
