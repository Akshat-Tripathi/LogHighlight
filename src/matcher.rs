use regex;

use crate::{
    colour,
    config::{ParseError, Segment},
};

pub(crate) struct Matcher {
    regex: regex::Regex,
    colour: colour::Colour,
}

impl Matcher {
    pub fn new(regex: String, colour: String) -> Result<Matcher, ParseError> {
        Ok(Matcher {
            regex: regex::Regex::new(&regex).or(Err(ParseError::InvalidRegex(regex)))?,
            colour: colour::Colour::try_from(colour)?,
        })
    }
}

impl TryFrom<Segment> for Vec<Matcher> {
    type Error = ParseError;

    fn try_from(s: Segment) -> Result<Self, Self::Error> {
        let mut v = Vec::new();
        for (ident, colour) in s.identifiers().iter().zip(s.colours()) {
            v.push(Matcher::new(ident.to_string(), colour.to_string())?);
        }
        Ok(v)
    }
}
