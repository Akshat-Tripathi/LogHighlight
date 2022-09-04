use colour::Colour;

mod config;
mod colour;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = std::fs::File::open("example_conf.yaml")?;
    let cfg = config::parse_config_from_reader(f);
    match cfg {
        Ok(cfg) => println!("{:?}", cfg),
        Err(e) => eprintln!("{}", e),
    }

    println!("{}", colour::colour_string("hello there".to_string(), colour::Colour::new(0, 255, 255)));

    Ok(())
}
