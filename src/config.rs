use log::LevelFilter;
use serde::{Deserialize, Serialize};

const CONFIG_PATH: &str = "config.json";

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    match load_config_from_env() {
        Ok(config) => Ok(config),
        Err(_) => match std::path::Path::new(CONFIG_PATH).exists() {
            true => load_config_from_json(),
            false => {
                let default_config = Config::default();
                let file = std::fs::File::create(CONFIG_PATH)?;
                serde_json::to_writer_pretty(file, &default_config)?;
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "You need to set a valid YGG_USERNAME and YGG_PASSWORD in environment variables or edit the created config.json file.",
                )))
            }
        },
    }
}

fn load_config_from_json() -> Result<Config, Box<dyn std::error::Error>> {
    if std::path::Path::new(CONFIG_PATH).exists() {
        let file = std::fs::File::open(CONFIG_PATH)?;
        let reader = std::io::BufReader::new(file);
        let config: Config = serde_json::from_reader(reader)?;
        let default_config = Config::default();
        if config.username == default_config.username || config.password == default_config.password
        {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Please set a valid YGG_USERNAME and YGG_PASSWORD in config.json.",
            )));
        }
        Ok(config)
    } else {
        let default_config = Config::default();
        let file = std::fs::File::create(CONFIG_PATH)?;
        serde_json::to_writer_pretty(file, &default_config)?;
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Configuration file not found, created a default one.",
        )))
    }
}

fn load_config_from_env() -> Result<Config, std::io::Error> {
    let username = std::env::var("YGG_USERNAME").map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "YGG_USERNAME env var is undefined",
        )
    })?;

    let password = std::env::var("YGG_PASSWORD").map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "YGG_PASSWORD env var is undefined",
        )
    })?;

    let bind_ip = std::env::var("BIND_IP").unwrap_or("0.0.0.0".to_string());

    let bind_port = std::env::var("BIND_PORT")
        .unwrap_or("8715".to_string())
        .parse::<u16>()
        .map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "BIND_PORT must be a valid number between 1 and 65535",
            )
        })?;

    let log_level = std::env::var("LOG_LEVEL")
        .unwrap_or("debug".to_string())
        .parse::<LevelFilter>()
        .map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "LOG_LEVEL must be a valid log level (off, error, warn, info, debug, trace)",
            )
        })?;

    let tmdb_token = std::env::var("TMDB_TOKEN").ok();
    let ygg_domain = std::env::var("YGG_DOMAIN").ok();

    Ok(Config {
        username,
        password,
        bind_ip,
        bind_port,
        log_level,
        tmdb_token,
        ygg_domain,
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub username: String,
    pub password: String,
    pub bind_ip: String,
    pub bind_port: u16,
    #[serde(with = "log_level_serde")]
    pub log_level: LevelFilter,
    pub tmdb_token: Option<String>,
    pub ygg_domain: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            username: "your_ygg_username".to_string(),
            password: "your_ygg_password".to_string(),
            bind_ip: "0.0.0.0".to_string(),
            bind_port: 8715,
            log_level: LevelFilter::Debug,
            tmdb_token: None,
            ygg_domain: None,
        }
    }
}

mod log_level_serde {
    use log::LevelFilter;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(level: &LevelFilter, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match *level {
            LevelFilter::Off => "off",
            LevelFilter::Error => "error",
            LevelFilter::Warn => "warn",
            LevelFilter::Info => "info",
            LevelFilter::Debug => "debug",
            LevelFilter::Trace => "trace",
        })
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<LevelFilter, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "off" => Ok(LevelFilter::Off),
            "error" => Ok(LevelFilter::Error),
            "warn" => Ok(LevelFilter::Warn),
            "info" => Ok(LevelFilter::Info),
            "debug" => Ok(LevelFilter::Debug),
            "trace" => Ok(LevelFilter::Trace),
            _ => Err(serde::de::Error::custom("Niveau de log invalide")),
        }
    }
}
