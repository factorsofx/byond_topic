extern crate byondtopic;
extern crate clap;

use byondtopic::topic;
use std::process::exit;
use clap::{App, Arg};

fn main() {
    let matches = App::new("ByondTopic")
        .version("1.0")
        .author("Collin Burroughs <collin@indyburroughs.net>")
        .about("Makes a topic request to the given BYOND server with the given text")
        .arg(Arg::with_name("timeout")
            .short("t")
            .long("timeout")
            .value_name("TIME")
            .help("Timeout length, in milliseconds")
            .takes_value(true))
        .arg(Arg::with_name("ADDR")
            .help("Address to connect to")
            .required(true)
            .index(1))
        .arg(Arg::with_name("TOPIC")
            .help("The topic to send to the given server")
            .required(true)
            .index(2))
        .get_matches();
    
    let timeout = match matches.value_of("timeout").unwrap_or("5000").parse::<usize>() {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(2)
        }
    };

    let addr = matches.value_of("ADDR").unwrap();
    let topic_req = matches.value_of("TOPIC").unwrap();

    let topic_resp = topic(addr, topic_req, timeout);
    match topic_resp {
        Ok(result) => println!("{}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(3);
        }
    }
}