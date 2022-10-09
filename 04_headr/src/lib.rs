use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::{self, BufRead, BufReader};
use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
    .version("0.1.0")
    .author("Chandler Wilkerson <chwilk@gmail.com>")
    .about("Rust head")
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
        .short("n")
        .long("lines")
        .help("print the first NUM lines of each file, default 10")
        .default_value("10")
        .takes_value(true),
    )
    .arg(
        Arg::with_name("bytes")
        .value_name("BYTES")
        .short("c")
        .long("bytes")
        .help("print the first NUM bytes of each file")
        .takes_value(true)
        .conflicts_with("lines"),
    ).get_matches();

    let lines = matches
        .value_of("lines")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?;

    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines.unwrap(),
        bytes
    })
}

fn parse_positive_int(s: &str) -> MyResult<usize> {
    match s.parse::<usize>() {
        Ok(i) if i > 0 => Ok(i),
        _ => Err(From::from(s)),
    }
}

#[test]
fn test_parse_positive_int() {
    // 3 should work
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    // String should error
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    // 0 should error
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}

pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.files.len();
    let mut file_num = 0;
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(mut fh) => {
                if num_files > 1 {
                    println! ("{}==> {} <==",
                    if file_num > 0 { "\n" } else { "" },
                    filename);
                    file_num += 1;
                }
                match config.bytes {
                    Some(num) => {
                        let mut handle = fh.take(num as u64);
                        let mut buf = vec![0; num ];
                        let bytes_read = handle.read(&mut buf)?;
                        print!("{}", 
                            String::from_utf8_lossy(&buf[..bytes_read]));
                    },
                    None => {
                        let mut line = String::new();
                        for _i in 0..config.lines {
                            let bytes = fh.read_line(&mut line)?;
                            if bytes == 0 { break; }
                            print!("{}", line);
                            line.clear();
                        }
                    },
                };
            }
        };
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}