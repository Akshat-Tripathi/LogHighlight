use std::collections::BTreeMap;
use std::fmt;
use std::io;

use serde::Deserialize;

pub(crate) type Config = BTreeMap<String, Profile>;
pub(crate) type Profile = BTreeMap<String, Segment>;

#[derive(Debug, Deserialize)]
pub(crate) struct Segment {
    identifiers: Vec<String>,
    colours: Vec<String>,
}

#[derive(Debug)]
pub(crate) enum ConfigError {
    MismatchedIdentColours {
        profile_name: String,
        segment_name: String,
    },
    CouldNotParse(serde_yaml::Error),
}

impl std::error::Error for ConfigError {}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::MismatchedIdentColours {
                segment_name,
                profile_name,
            } => write!(
                f,
                "Segment '{}' must have the same number of colours and identifiers in profile '{}'",
                segment_name, profile_name
            ),
            ConfigError::CouldNotParse(error) => {
                write!(f, "Couldn't parse config, error: {}", error)
            }
        }
    }
}

impl Segment {
    pub fn is_valid(&self) -> bool {
        self.identifiers.len() == self.colours.len()
    }
}

pub(crate) fn parse_config_from_reader<R>(reader: R) -> Result<Config, ConfigError>
where
    R: io::Read,
{
    let cfg: Config = serde_yaml::from_reader(reader).map_err(|e| ConfigError::CouldNotParse(e))?;

    for (name, profile) in cfg.iter() {
        for (k, v) in profile.iter() {
            if !v.is_valid() {
                return Err(ConfigError::MismatchedIdentColours {
                    profile_name: name.clone(),
                    segment_name: k.clone(),
                });
            }
        }
    }

    Ok(cfg)
}
