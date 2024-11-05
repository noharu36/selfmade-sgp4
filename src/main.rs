use self_made_sgp4::tle;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let elements = tle::tle_parse(filename)?;

    println!("{:?}", elements[0]);

    Ok(())
}
