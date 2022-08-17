use std::env;
use std::process;
use minigrep::ConfigParams;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let config = ConfigParams::new(&arguments).unwrap_or_else(|err| {
        eprintln!("Encountered an error: {:?}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error {:?}", e);
        process::exit(1);
    }
}

