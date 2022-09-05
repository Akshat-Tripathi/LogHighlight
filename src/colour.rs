use crate::config::ParseError;

#[derive(Debug)]
pub(crate) struct Colour {
    r: u8,
    g: u8,
    b: u8,
}

impl Colour {
    pub(crate) fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

pub(crate) fn colour_string(string: String, colour: Colour) -> String {
    format!(
        "\x1b[38;2;{};{};{}m{}\x1b[0m\n",
        colour.r, colour.g, colour.b, string
    )
}

impl TryFrom<String> for Colour {
    type Error = ParseError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.len() != 6 {
            return Err(ParseError::InvalidColour(s));
        }
        let hex = u32::from_str_radix(&s, 16).or(Err(ParseError::InvalidColour(s)))?;
        let r = ((hex & 0xFF0000) >> 16) as u8;
        let g = ((hex & 0x00FF00) >> 8) as u8;
        let b = (hex & 0x0000FF) as u8;
        Ok(Colour::new(r, g, b))
    }
}
