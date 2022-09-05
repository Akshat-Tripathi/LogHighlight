use std::collections::BTreeMap;
use std::fmt;
use std::io;

use serde::Deserialize;

pub(crate) type Config = BTreeMap<String, Profile>;
pub(crate) type Profile = BTreeMap<String, Segment>;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Segment {
    identifiers: Vec<String>,
    colours: Vec<String>,
}

#[derive(Debug)]
pub(crate) enum ParseError {
    MismatchedIdentColours {
        profile_name: String,
        segment_name: String,
    },
    InvalidRegex(String),
    InvalidColour(String),
    CouldNotParse(serde_yaml::Error),
}

impl std::error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::MismatchedIdentColours {
                segment_name,
                profile_name,
            } => write!(
                f,
                "Segment '{}' must have the same number of colours and identifiers in profile '{}'",
                segment_name, profile_name
            ),
            ParseError::InvalidRegex(r) => write!(f, "Invalid regex: {}", r),
            ParseError::InvalidColour(c) => write!(f, "Invalid colour: {}", c),
            ParseError::CouldNotParse(error) => {
                write!(f, "Couldn't parse config, error: {}", error)
            }
        }
    }
}

impl Segment {
    pub fn is_valid(&self) -> bool {
        self.identifiers.len() == self.colours.len()
    }

    pub(crate) fn identifiers(&self) -> &[String] {
        self.identifiers.as_ref()
    }

    pub(crate) fn colours(&self) -> &[String] {
        self.colours.as_ref()
    }
}

pub(crate) fn parse_config_from_reader<R>(reader: R) -> Result<Config, ParseError>
where
    R: io::Read,
{
    let cfg: Config = serde_yaml::from_reader(reader).map_err(|e| ParseError::CouldNotParse(e))?;

    for (name, profile) in cfg.iter() {
        for (k, v) in profile.iter() {
            if !v.is_valid() {
                return Err(ParseError::MismatchedIdentColours {
                    profile_name: name.clone(),
                    segment_name: k.clone(),
                });
            }
        }
    }

    Ok(cfg)
}
