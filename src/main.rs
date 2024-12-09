use std::{env,process};
use colored::*;

use minigrep::Config;

fn main() {
    let args:Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem passing arguments: {}",err.red().bold());
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config){
        eprintln!("Application Error: {e}");
        process::exit(1);
    }
}
