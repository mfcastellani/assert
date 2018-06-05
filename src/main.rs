extern crate clap;
extern crate console;

use clap::{App, SubCommand, Arg};
use console::Style;
use std::str::FromStr;

pub fn check_eq(v1: &str, v2: &str) -> Result<(), ()> {
    if String::from_str(v1).eq(&String::from_str(v2)) {
        Ok(())
    } else {
        Err(())
    }
}

pub fn check_diff(v1: &str, v2: &str) -> Result<(), ()> {
    if String::from_str(v1).eq(&String::from_str(v2)) {
        Err(())
    } else {
        Ok(())
    }
}

fn main() {
    let matches = App::new("assert")
        .version("1.0.0")
        .author("Marcelo Castellani <marcelofc.rock@gmail.com>")
        .about("Compare two strings and return")
        .subcommand(SubCommand::with_name("eq")
                    .about("Compare both values. If equal return 0, otherwise 1.")
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
        .subcommand(SubCommand::with_name("diff")
                    .about("Compare both values. If not equal return 0, otherwise 1.")
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
            match check_eq(v1, v2) {
                Ok(_) => println!("Both are equal"),
                Err(_) => println!("Both are not equal")
            }
        } else {
            ::std::process::exit(match check_eq(v1, v2) {
                Ok(_) => 0,
                Err(_) => 1
            });
        }
    }

    if let Some(matches) = matches.subcommand_matches("diff") {
        let v1 = matches.value_of("Value1").unwrap();
        let v2 = matches.value_of("Value2").unwrap();

        if matches.is_present("print") {
            let cyan = Style::new().cyan();
            println!("Checking if {} is not equal to {}", cyan.apply_to(v1), cyan.apply_to(v2));
            match check_diff(v1, v2) {
                Ok(_) => println!("Both are not equal"),
                Err(_) => println!("Both are equal")
            }
        } else {
            ::std::process::exit(match check_diff(v1, v2) {
                Ok(_) => 0,
                Err(_) => 1
            });
        }
    }
}


#[cfg(test)]
mod tests {
    extern crate docmatic;
    use super::*;

    #[test]
    fn test_readme() {
        docmatic::assert_file("README.md");
    }

    #[test]
    fn test_eq_ok() {
        assert_eq!(Ok(()), check_eq("Test", "Test"));
    }

    #[test]
    fn test_eq_err() {
        assert_eq!(Err(()), check_eq("Test1", "Test"));
    }

    #[test]
    fn test_diff_ok() {
        assert_eq!(Ok(()), check_diff("Test1", "Test"));
    }

    #[test]
    fn test_diff_err() {
        assert_eq!(Err(()), check_diff("Test", "Test"));
    }
}

