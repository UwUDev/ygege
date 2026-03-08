use log::LevelFilter;
use serde::{Deserialize, Serialize};

const CONFIG_PATH: &str = "config.json";

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    if let Ok(config) = load_config_from_env() {
        return Ok(config);
    }
    if std::path::Path::new(CONFIG_PATH).exists() {
        load_config_from_json()
    } else {
        let default_config = Config::default();
        let file = std::fs::File::create(CONFIG_PATH)?;
        serde_json::to_writer_pretty(file, &default_config)?;
        Ok(default_config)
    }
}

fn load_config_from_json() -> Result<Config, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(CONFIG_PATH)?;
    let reader = std::io::BufReader::new(file);
    let config: Config = serde_json::from_reader(reader)?;
    Ok(config)
}

fn load_config_from_env() -> Result<Config, std::io::Error> {
    const ENV_KEYS: &[&str] = &[
        "BIND_IP",
        "BIND_PORT",
        "LOG_LEVEL",
        "TMDB_TOKEN",
        "USE_TOR",
        "TOR_PROXY",
    ];
    if !ENV_KEYS.iter().any(|k| std::env::var(k).is_ok()) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No env vars set, falling back to config.json",
        ));
    }

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

    let use_tor = std::env::var("USE_TOR")
        .unwrap_or("false".to_string())
        .to_lowercase()
        == "true";

    let tor_proxy = std::env::var("TOR_PROXY")
        .ok()
        .filter(|s| !s.is_empty())
        .or_else(|| {
            if use_tor {
                Some("127.0.0.1:9050".to_string())
            } else {
                None
            }
        });

    Ok(Config {
        bind_ip,
        bind_port,
        log_level,
        tmdb_token,
        use_tor,
        tor_proxy,
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub bind_ip: String,
    pub bind_port: u16,
    #[serde(with = "log_level_serde")]
    pub log_level: LevelFilter,
    pub tmdb_token: Option<String>,
    #[serde(default = "default_use_tor")]
    pub use_tor: bool,
    #[serde(default)]
    pub tor_proxy: Option<String>,
}

fn default_use_tor() -> bool {
    false
}

impl Default for Config {
    fn default() -> Self {
        Config {
            bind_ip: "0.0.0.0".to_string(),
            bind_port: 8715,
            log_level: LevelFilter::Debug,
            tmdb_token: None,
            use_tor: false,
            tor_proxy: Some("127.0.0.1:9050".to_string()),
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
