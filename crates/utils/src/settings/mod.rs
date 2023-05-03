use crate::{
    location_info,
    settings::structs::{
        CaptchaConfig, DatabaseConfig, EmailConfig, RateLimitConfig, Settings, SetupConfig,
        SocialProviderConfig, SocialProviderUrl, TokenConfig, TokenPrefix, Validation,
    },
    AuthError,
};
use anyhow::{anyhow, Context};
use deser_hjson::from_str;
use log::warn;
use merge::Merge;
use std::{env, fs, io::Error, net::IpAddr, sync::RwLock};

pub(crate) mod defaults;
pub mod structs;

static CONFIG_FILE: &str = "config/config.hjson";

lazy_static! {
    static ref SETTINGS: RwLock<Settings> = RwLock::new(match Settings::init() {
        Ok(c) => c,
        Err(e) => {
            warn!(
                "Couldn't load settings file, using default settings.\n{}",
                e
            );
            Settings::default()
        }
    });
}

impl Settings {
    /// Reads config from the files and environment.
    /// First, defaults are loaded from CONFIG_FILE_DEFAULTS, then these values can be overwritten
    /// from CONFIG_FILE (optional). Finally, values from the environment (with prefix AUTH) are
    /// added to the config.
    ///
    /// Note: The env var `AUTH_DATABASE_URL` is parsed in
    /// `db_queries/src/lib.rs::get_database_url_from_env()`
    fn init() -> Result<Self, AuthError> {
        // Read the config file
        let mut custom_config = from_str::<Settings>(&Self::read_config_file()?)?;

        // Merge with env vars
        custom_config.merge(envy::prefixed("AUTH_").from_env::<Settings>()?);

        // Merge with default
        custom_config.merge(Settings::default());

        if custom_config.hostname == Settings::default().hostname {
            return Err(anyhow!("Hostname variable is not set!").into());
        }

        Ok(custom_config)
    }

    /// Returns the config as a struct.
    pub fn get() -> Self {
        SETTINGS.read().expect("read config").to_owned()
    }

    pub fn get_database_url(&self) -> String {
        let conf = self.database();
        format!(
            "postgres://{}:{}@{}:{}/{}",
            conf.user, conf.password, conf.host, conf.port, conf.database,
        )
    }

    pub fn get_config_location() -> String {
        env::var("AUTH_CONFIG_LOCATION").unwrap_or_else(|_| CONFIG_FILE.to_string())
    }

    pub fn read_config_file() -> Result<String, Error> {
        fs::read_to_string(Self::get_config_location())
    }

    /// Returns either "http" or "https", depending on tls_enabled setting
    pub fn get_protocol_string(&self) -> &'static str {
        if let Some(tls_enabled) = self.tls_enabled {
            if tls_enabled {
                "https"
            } else {
                "http"
            }
        } else {
            "http"
        }
    }

    /// Returns something like `http://localhost` or `https://[subdomain].hicampi.com`,
    /// with the correct protocol and hostname.
    pub fn get_protocol_and_hostname(&self) -> String {
        println!("get_protocol_and_hostname");
        format!("{}://{}", self.get_protocol_string(), self.hostname())
    }

    /// When running the test setup in `api_tests/`, the `hostname`
    /// variable will be like `auth-alpha:8541`. This method removes the port and returns
    /// `auth-alpha` instead. It has no effect in production.
    pub fn get_hostname_without_port(&self) -> Result<String, anyhow::Error> {
        Ok(self
            .hostname()
            .split(':')
            .collect::<Vec<&str>>()
            .first()
            .context(location_info!())?
            .to_string())
    }

    pub fn save_config_file(data: &str) -> Result<String, AuthError> {
        fs::write(CONFIG_FILE, data)?;

        // Reload the new settings
        let mut new_settings = SETTINGS.write().expect("write config");
        *new_settings = match Settings::init() {
            Ok(c) => c,
            Err(e) => panic!("{}", e),
        };

        Ok(Self::read_config_file()?)
    }

    pub fn multi_instance_mode(&self) -> bool {
        self.multi_instance_mode.unwrap_or_default()
    }

    pub fn database(&self) -> DatabaseConfig {
        self.database.to_owned().unwrap_or_default()
    }
    pub fn hostname(&self) -> String {
        self.hostname.to_owned().unwrap_or_default()
    }
    pub fn bind(&self) -> IpAddr {
        self.bind.expect("return bind address")
    }
    pub fn port(&self) -> u16 {
        self.port.unwrap_or_default()
    }
    pub fn tls_enabled(&self) -> bool {
        self.tls_enabled.unwrap_or_default()
    }
    pub fn jwt_secret(&self) -> String {
        self.jwt_secret.to_owned().unwrap_or_default()
    }
    pub fn files_url(&self) -> String {
        self.files_url.to_owned().unwrap_or_default()
    }
    pub fn iframely_url(&self) -> String {
        self.iframely_url.to_owned().unwrap_or_default()
    }
    pub fn rate_limit(&self) -> RateLimitConfig {
        self.rate_limit.to_owned().unwrap_or_default()
    }
    pub fn captcha(&self) -> CaptchaConfig {
        self.captcha.to_owned().unwrap_or_default()
    }
    pub fn email(&self) -> Option<EmailConfig> {
        self.email.to_owned()
    }
    pub fn setup(&self) -> Option<SetupConfig> {
        self.setup.to_owned()
    }
    pub fn session_key(&self) -> [u8; 32] {
        self.session_key.expect("return session key")
    }
    pub fn social_provider(&self) -> SocialProviderConfig {
        self.social_provider.to_owned().unwrap_or_default()
    }
    pub fn social_provider_url(&self) -> SocialProviderUrl {
        self.social_provider_url.to_owned().unwrap_or_default()
    }
    pub fn config_at(&self) -> TokenConfig {
        self.access_token.to_owned().unwrap()
    }
    pub fn config_it(&self) -> TokenConfig {
        self.id_token.to_owned().unwrap()
    }
    pub fn config_rt(&self) -> TokenConfig {
        self.refresh_token.to_owned().unwrap()
    }
    pub fn config_st(&self) -> TokenConfig {
        self.state_token.to_owned().unwrap()
    }
}
