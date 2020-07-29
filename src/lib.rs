use json;
use std::fs;
use std::time::Duration;
use dbus::blocking::Connection;

#[derive(Debug)]
pub struct Config {
    pub loglevel: u8,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str>  {
        
        if args.len() < 2 {
            return Err("Not enough arguments!");
        }

        let path = &args[1];
        let content = fs::read_to_string(path).expect("Error reading file!");
        let j = json::parse( &content ).expect("File is not a valid JSON!");

        Ok(Config { loglevel: j["loglevel"].as_u8().expect("loglevel is not a valid number") })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    
    println!("The current configuration is: {:?}", config);
    example()?;
    Ok(())
}

fn example() -> Result<(), Box<dyn std::error::Error>> {
    // D-Bus query
    let conn = Connection::new_session()?;
    let proxy = conn.with_proxy("org.freedesktop.DBus", "/", Duration::from_millis(5000));
    let (names,): (Vec<String>,) = proxy.method_call("org.freedesktop.DBus", "ListNames", ())?;

    // Let's print all the names to stdout.
    for name in names { println!("{}", name); }

    Ok(())
}
