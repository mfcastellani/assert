extern crate clap;
extern crate console;

use clap::{App, SubCommand, Arg};
use console::Style;
use std::str::FromStr;

fn assert_eq(v1: &str, v2: &str) -> Result<(), ()> {
    if String::from_str(v1).eq(&String::from_str(v2)) {
        Ok(())
    } else {
        Err(())
    }
}

fn main() {
    let matches = App::new("assert")
        .version("1.0.0")
        .author("Marcelo Castellani <marcelofc.rock@gmail.com>")
        .about("Compare two strings and return")
        .subcommand(SubCommand::with_name("eq")
                    .about("Test if both values are equal")
                    .arg(Arg::with_name("Value1")
                         .required(true)
                         .takes_value(true)
                         .index(1)
                         .help("First value"))
                    .arg(Arg::with_name("Value2")
                         .required(true)
                         .takes_value(true)
                         .index(2)
                         .help("Second value"))
                    .arg_from_usage("-p, --print 'Print comparison instead return'"))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("eq") {
        let v1 = matches.value_of("Value1").unwrap();
        let v2 = matches.value_of("Value2").unwrap();

        if matches.is_present("print") {
            let cyan = Style::new().cyan();
            println!("Checking if {} is equal to {}", cyan.apply_to(v1), cyan.apply_to(v2));
            match assert_eq(v1, v2) {
                Ok(_) => println!("Both are equal"),
                Err(_) => println!("Both are not equal")
            }
        } else {
            ::std::process::exit(match assert_eq(v1, v2) {
                Ok(_) => 0,
                Err(_) => 1
            });
        }
    }
}

