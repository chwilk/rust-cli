use std::error::Error;
use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[derive(Debug)]
struct Data {
    filename: String,
    lines: usize,
    words: usize,
    bytes: usize,
    chars: usize,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
    .version("0.1.0")
    .author("Chandler Wilkerson <chwilk@gmail.com>")
    .about("Rust word count")
    .arg(
        Arg::with_name("files")
        .value_name("FILES")
        .help("Files or - for STDIN")
        .default_value("-")
        .min_values(0),
    )
    .arg(
        Arg::with_name("lines")
        .value_name("LINES")
        .short("l")
        .long("lines")
        .help("print the newline counts")
        .takes_value(false),
    )
    .arg(
        Arg::with_name("words")
        .value_name("WORDS")
        .short("w")
        .long("words")
        .help("print the word counts")
        .takes_value(false),
    )
    .arg(
        Arg::with_name("bytes")
        .value_name("BYTES")
        .short("c")
        .long("bytes")
        .help("print the byte counts")
        .takes_value(false),
    )
    .arg(
        Arg::with_name("chars")
        .value_name("CHARS")
        .short("m")
        .long("chars")
        .help("print the character counts")
        .conflicts_with("bytes")
        .takes_value(false),
    )
    .get_matches();

    let mut lines = matches.is_present("lines");
    let mut words = matches.is_present("words");
    let mut bytes = matches.is_present("bytes");
    let chars = matches.is_present("chars");

    if [lines, words, bytes, chars].iter().all(|v| v==&false) {
        lines = true;
        words = true;
        bytes = true;
    }

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines,
        words,
        bytes,
        chars,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut data = Vec::new();
    for filename in &config.files {
        match open(&filename) {
            Err(e) => eprintln!("Failed to open {}: {}", filename, e),
            Ok(mut file) => data.push(count(&mut file, &filename).unwrap()),
        }
    }
    let mut sum = Data {
        filename: "total".to_string(),
        lines: 0,
        words: 0,
        bytes: 0,
        chars: 0,
    };
    if data.len() > 1 {
        for datum in &data {
            sum.lines += datum.lines;
            sum.words += datum.words;
            sum.bytes += datum.bytes;
            sum.chars += datum.chars;
        }
    }

    for datum in &data {
        println!("{}", print_data(&config, &datum)?);
    }
    if data.len() > 1 {
        println!("{}", print_data(&config, &sum)?);
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn count(mut file: impl BufRead, filename: &str) -> MyResult<Data> {
    let mut lines = 0;
    let mut words = 0;
    let mut chars = 0;
    let mut bytes = 0;

    let mut line = String::new();
    loop {
        let b = file.read_line(&mut line)?;
        if b == 0 {break;}

        lines += 1;
        chars += line.chars().count();
        words += line.split_whitespace().count();
        bytes += b;
        line.clear();
    }
    Ok(Data {
        filename: filename.to_string(),
        lines,
        words,
        chars,
        bytes,
    })
}

fn print_data(config: &Config, data: &Data) -> MyResult<String> {
    let mut output = String::new();
    if config.lines {
        output.push_str(&format!("{:8}", data.lines));
    }
    if config.words {
        output.push_str(&format!("{:8}", data.words));
    }
    if config.chars {
        output.push_str(&format!("{:8}", data.chars));
    }
    if config.bytes {
        output.push_str(&format!("{:8}", data.bytes));
    }
    if data.filename != "-" {
        output.push_str(&format!(" {}", data.filename));
    }
    Ok(output)
}