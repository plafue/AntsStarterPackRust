use config::read_config;
use std::io::*;
mod config;

fn main() {
    let stdin = stdin();
    let config = read_config(&mut BufReader::new(stdin.lock()));
    println!("//NON STANDARD PROTOCOL OUTPUT: Config: {:?}", config);
    println!("go");
}
