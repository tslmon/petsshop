use crate::settings::{
    CaptchaConfig, DatabaseConfig, RateLimitConfig, Settings, SocialProviderConfig,
    SocialProviderUrl, TokenConfig, TokenPrefix, Validation,
};
use rand::Rng;
use std::net::{IpAddr, Ipv4Addr};

use super::structs::TokenAlgorithm;
impl Default for Settings {
    fn default() -> Self {
        Self {
            database: Some(DatabaseConfig::default()),
            rate_limit: Some(RateLimitConfig::default()),
            captcha: Some(CaptchaConfig::default()),
            email: None,
            setup: None,
            hostname: None,
            bind: Some(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))),
            port: Some(8536),
            multi_instance_mode: Some(false),
            tls_enabled: Some(true),
            jwt_secret: Some("changeme".into()),
            files_url: Some("http://localhost:8080".into()),
            iframely_url: Some("http://iframely".into()),
            session_key: Some(rand::thread_rng().gen::<[u8; 32]>()),
            social_provider: Some(SocialProviderConfig::default()),
            social_provider_url: Some(SocialProviderUrl::default()),
            access_token: Some(TokenConfig {
                prefix: TokenPrefix::AT,
                validation: Validation {
                    leeway: 3600,
                    validate_exp: true,
                    validate_nbf: false,
                    aud: Some(vec!["Me".to_string(), "You".to_string()]),
                    iss: Some(String::from("localhost")),
                    sub: None,
                    algorithm: vec![String::from("HS256")],
                },
                algorithm: TokenAlgorithm {
                    algorithm: String::from("HS256"),
                    private_key: String::from("access_key"),
                    public_key: None,
                },
            }),
            id_token: Some(TokenConfig {
                prefix: TokenPrefix::IT,
                validation: Validation {
                    leeway: 0,
                    validate_exp: true,
                    validate_nbf: false,
                    aud: Some(vec!["Me".to_string(), "You".to_string()]),
                    iss: Some(String::from("localhost")),
                    sub: None,
                    algorithm: vec![String::from("HS256")],
                },
                algorithm: TokenAlgorithm {
                    algorithm: String::from("HS256"),
                    private_key: String::from("id_key"),
                    public_key: None,
                },
            }),
            refresh_token: Some(TokenConfig {
                prefix: TokenPrefix::RT,
                validation: Validation {
                    leeway: 1800,
                    validate_exp: true,
                    validate_nbf: false,
                    aud: Some(vec!["Me".to_string(), "You".to_string()]),
                    iss: Some(String::from("localhost")),
                    sub: None,
                    algorithm: vec![String::from("HS256")],
                },
                algorithm: TokenAlgorithm {
                    algorithm: String::from("HS256"),
                    private_key: String::from("refresh_key"),
                    public_key: None,
                },
            }),
            state_token: Some(TokenConfig {
                prefix: TokenPrefix::ST,
                validation: Validation {
                    leeway: 60,
                    validate_exp: true,
                    validate_nbf: false,
                    aud: Some(vec!["Me".to_string(), "You".to_string()]),
                    iss: Some(String::from("localhost")),
                    sub: None,
                    algorithm: vec![String::from("HS256")],
                },
                algorithm: TokenAlgorithm {
                    algorithm: String::from("HS256"),
                    private_key: String::from("state_key"),
                    public_key: None,
                },
            }),
            password_policy: None,
        }
    }
}

// impl Default for TokenConfig {
//     fn default() -> Self {
//         Self {
//             user: "auth".into(),
//             password: "Welc0meHicamp1".into(),
//             host: "localhost".into(),
//             port: 5432,
//             database: "auth".into(),
//             pool_size: 5,
//         }
//     }
// }

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            user: "postgres".into(),
            password: "1234".into(),
            host: "localhost".into(),
            port: 5432,
            database: "petsshop".into(),
            pool_size: 5,
        }
    }
}

impl Default for CaptchaConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            difficulty: "medium".into(),
        }
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            signups: 100000,
            signups_per_second: 1,
            signins: 100000,
            signins_per_second: 1,
            invites: 100000,
            invites_per_second: 1,
            settings: 3,
            settings_per_second: 1,
            recoveries: 6,
            recoveries_per_second: 1,
            verify: 6,
            verify_per_second: 1,
            otp: 6,
            otp_per_second: 1,
            token: 6,
            token_per_second: 1,
            user: 6,
            user_per_second: 1,
            signout: 6,
            signout_per_second: 1,
            authorize: 6,
            authorize_per_second: 1,
            callback: 6,
            callback_per_second: 1,
            audit: 6,
            audit_per_second: 1,
            users: 6,
            users_per_second: 1,
            generate_link: 6,
            generate_link_per_second: 1,
            saml: 6,
            saml_per_second: 1,
            tenants: 6,
            tenants_per_second: 1,
        }
    }
}

impl Default for SocialProviderConfig {
    fn default() -> Self {
        Self {
            github_enabled: Some(true),
            github_client_id: "f2efc0c5073f0921bc3f".into(),
            github_client_secret: "4fb9426e0035329155fe24c6967b5fe42bb5c1fc".into(),
            google_enabled: Some(true),
            google_client_id:
                "779463943063-6qpp8kv7tmf6h7l126insec1s2btimo4.apps.googleusercontent.com".into(),
            google_client_secret: "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".into(),
            fb_enabled: Some(true),
            fb_client_id: "1419824648433636".into(),
            fb_client_secret: "686f4fe466fedaf201547e3384d252d7".into(),
        }
    }
}

impl Default for SocialProviderUrl {
    fn default() -> Self {
        Self {
            github_auth_url: "https://github.com/login/oauth/authorize".into(),
            github_token_url: "https://github.com/login/oauth/access_token".into(),
            google_auth_url: "https://accounts.google.com/o/oauth2/v2/auth".into(),
            google_token_url: "https://oauth2.googleapis.com/token".into(),
            fb_auth_url: "https://www.facebook.com/v12.0/dialog/oauth".into(),
            fb_token_url: "https://graph.facebook.com/v12.0/oauth/access_token".into(),
        }
    }
}
