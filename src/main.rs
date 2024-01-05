use std::env::args;
use std::error::Error;
use colored::*;

fn main() {
    let _run = Colare::print_args();
}

struct Colare;

impl Colare {
    fn print_args() -> Result<(), Box<dyn Error>> {
    
        let argc: Vec<String> = args().collect();
    
        let msg: &str = argc[1].trim();
    
        println!("{}", msg.to_string().truecolor( 79, 255, 112 ).bold());

        Ok(())
    }
}

