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
