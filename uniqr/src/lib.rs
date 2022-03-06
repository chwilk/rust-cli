use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::{self, BufRead, BufReader};
use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("uniqr")
    .version("0.1.0")
    .author("Chandler Wilkerson <chwilk@gmail.com>")
    .about("Rust uniq")
    .arg(
        Arg::with_name("in_file")
        .value_name("INFILE")
        .help("input file")
        .default_value("-")
    )
    .arg(
        Arg::with_name("out_file")
        .value_name("OUTFILE")
        .help("output file")
        .requires("in_file")
    )
    .arg(
        Arg::with_name("count")
        .long("count")
        .short("c")
        .takes_value(false)
        .help("prefix lines by the number of occurrences")
    )
    .get_matches();

    Ok(Config {
        in_file: matches.value_of("in_file").unwrap().to_string(),
        out_file: matches.value_of("out_file").map(String::from),
        count: matches.is_present("count"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);

    Ok(())
}