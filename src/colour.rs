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
