use clap::{App, Arg};
use std::{
    error::Error,
    fs::File,
    io::Read,
    io::{self, BufRead, BufReader, Write, BufWriter},
};

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
    let mut file = open(&config.in_file)
        .map_err(|e| format!("{}: {}", config.in_file, e))?;
    let mut out_file = open_out(config.out_file)
        .map_err(|e_o| format!("{}", e_o))?;
    let mut line = String::new();
    let mut last = String::new();
    let mut count = 0;
    let mut print = |count: u64, text: &str| -> MyResult<()>{
        if count > 0 {
            if config.count {
                write!(out_file, "{:>4} {}", count, text)?;
            } else {
                write!(out_file, "{}", text)?;
            }
        }
        Ok(())
    };
    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 { break; }
        if line.trim_end() == last.trim_end() {
            count += 1;
        } else {
            print(count, &last);
            count = 1;
            last.clone_from(&line);
        }
        line.clear();
    }
    print(count, &last);

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn open_out(out_file: Option<String>) -> MyResult<Box<dyn Write>> {
    match out_file {
        Some(filename) => Ok(Box::new(BufWriter::new(File::create(filename)?))),
        None => Ok(Box::new(BufWriter::new(io::stdout()))),
    }
}