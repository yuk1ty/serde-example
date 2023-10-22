use std::{collections::HashMap, error::Error, fmt::Display, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_yaml::Value;

#[derive(Debug)]
pub struct EasyError {
    details: String,
}

impl Display for EasyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for EasyError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Attribute(String);

#[derive(Debug)]
pub enum ConfigValue {
    String(String),
    Number(u64),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    path: PathBuf,
    #[serde(skip)]
    extra_config: HashMap<Attribute, ConfigValue>,
}

impl Config {
    pub fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let rdr = std::fs::File::open(&self.path)?;
        let deserialized: HashMap<String, Value> = serde_yaml::from_reader(&rdr)?;
        for (k, v) in deserialized {
            match v {
                Value::String(s) => {
                    self.extra_config
                        .insert(Attribute(k), ConfigValue::String(s));
                }
                Value::Number(n) => {
                    self.extra_config.insert(
                        Attribute(k),
                        ConfigValue::Number(n.as_u64().ok_or(EasyError {
                            details: "数値ではないものが入っているかもしれません。".into(),
                        })?),
                    );
                }
                _ => {}
            }
        }
        Ok(())
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rdr = std::fs::File::open("config/path.yaml").unwrap();
    let mut config_base: Config = serde_yaml::from_reader(&rdr)?;
    println!("{:?}", config_base);

    config_base.load()?;
    println!("{:?}", config_base);

    Ok(())
}
