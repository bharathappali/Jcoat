#[macro_use]
extern crate clap;
use std::process::exit;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use clap::App;

fn main() {
    let yaml_to_load = load_yaml!("../yaml/config.yml");
    let match_config = App::from_yaml(yaml_to_load).get_matches();
    if !match_config.is_present("file") {
	println!("No input files");
	exit(1);
    } else { 
	if let Some(file_to_analyse) = match_config.value_of("file") {
	    println!("File to analyse: {}", file_to_analyse);

	    let input = File::open(file_to_analyse).expect("File opening error");
	    let buffered = BufReader::new(input);

	    for line in buffered.lines() {
	        println!("{}", line.unwrap());
	    }
    	}
    }
}
