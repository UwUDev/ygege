use log::LevelFilter;
use serde::{Deserialize, Serialize};

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = "config.json";
    if std::path::Path::new(config_path).exists() {
        let file = std::fs::File::open(config_path)?;
        let reader = std::io::BufReader::new(file);
        let config: Config = serde_json::from_reader(reader)?;
        Ok(config)
    } else {
        let default_config = Config::default();
        let file = std::fs::File::create(config_path)?;
        serde_json::to_writer_pretty(file, &default_config)?;
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Configuration file not found, created a default one.",
        )))
    }
}

pub fn load_config_from_env() -> Result<Config, String> {
    let username = std::env::var("YGG_USERNAME")
        .map_err(|_| "YGG_USERNAME env var is undefined".to_string())?;

    let password = std::env::var("YGG_PASSWORD")
        .map_err(|_| "YGG_PASSWORD env var is undefined".to_string())?;

    let bind_ip = std::env::var("BIND_IP").unwrap_or("0.0.0.0".to_string());

    let bind_port = std::env::var("BIND_PORT")
        .unwrap_or("8080".to_string())
        .parse::<u16>()
        .map_err(|_| "BIND_PORT must be a valid u16 number".to_string())?;

    let log_level = std::env::var("LOG_LEVEL")
        .unwrap_or("debug".to_string())
        .parse::<LevelFilter>()
        .map_err(|_| {
            "LOG_LEVEL must be a valid log level (off, error, warn, info, debug, trace)".to_string()
        })?;

    Ok(Config {
        username,
        password,
        bind_ip,
        bind_port,
        log_level,
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
}

impl Default for Config {
    fn default() -> Self {
        Config {
            username: "your_ygg_username".to_string(),
            password: "your_ygg_password".to_string(),
            bind_ip: "0.0.0.0".to_string(),
            bind_port: 8080,
            log_level: LevelFilter::Debug,
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
