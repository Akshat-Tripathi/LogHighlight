use colour::Colour;

mod colour;
mod config;
mod matcher;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = std::fs::File::open("example_conf.yaml")?;
    let profile = "logs";

    let cfg = config::parse_config_from_reader(f)?;

    let profile = cfg.get(profile).unwrap();
    let mut all_matchers = Vec::new();
    for segment in profile.values() {
        let mut matchers = Vec::<matcher::Matcher>::try_from(segment.clone())?;
        all_matchers.append(&mut matchers);
    }

    println!(
        "{}",
        colour::colour_string("hello there".to_string(), colour::Colour::new(0, 255, 255))
    );

    Ok(())
}
