use std::env;
use std::process;
use linuxpoc::Config;

fn main() -> Result<(),Box<dyn std::error::Error>> {

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else( |err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    linuxpoc::run(config)?;
    Ok(())
}

