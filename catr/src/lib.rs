use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use clap::{App, Arg};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number: bool,
    number_nonblank: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        let mut i = 1;
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(fh) => for line in fh.lines() {
                let l = line.unwrap();
                if config.number || 
                 (config.number_nonblank && l.len() > 0) {
                    print!("{:6}\t", i);
                    i += 1;
                }
                println!("{}", l);

            }
        };
    }
    Ok(())
}

macro_rules! args_app {
    () => {{
    App::new("catr")
    .version("0.1.0")
    .author("Chandler Wilkerson <chwilk@gmail.com>")
    .about("Rust cat")
    .arg(
        Arg::with_name("files")
        .value_name("FILES")
        .help("Files or - for STDIN")
        .default_value("-")
        .min_values(0),
    )
    .arg(
        Arg::with_name("number")
        .short("n")
        .long("number")
        .help("number all output lines")
        .takes_value(false),
    )
    .arg(
        Arg::with_name("number_nonblank")
        .short("b")
        .long("number-nonblank")
        .help("number all nonempty output lines, overrides -n")
        .takes_value(false)
        .conflicts_with("number"),
    )
    }};
}

pub fn get_args() -> MyResult<Config> {
    let matches = args_app!().get_matches();
    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number: matches.is_present("number"),
        number_nonblank: matches.is_present("number_nonblank"),
    })
}

#[test]
fn check_args() {
    let mut values: Vec<&str>;

    // with no arguments
    let m1 = args_app!().get_matches_from(vec![
        "catr".to_string(),
    ]);

    values = m1.values_of("files").unwrap().collect();
    assert_eq!(values, ["-"]);

    // two files
    let m2 = args_app!().get_matches_from(vec![
        "catr".to_string(),
        "-n".to_string(),
        "foo".to_string(),
        "bar".to_string(),
    ]);

    values = m2.values_of("files").unwrap().collect();
    assert_eq!(values, ["foo", "bar"]);
    assert!(m2.is_present("number"));

}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}