use crate::EntryType::*;
use clap::{Command, Arg};
use regex::Regex;
use std::error::Error;
use walkdir::{DirEntry, WalkDir};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Eq, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link,
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("findr")
    .version("0.1.0")
    .author("Chandler Wilkerson <chwilk@gmail.com>")
    .about("Rust find")
    .arg(
        Arg::new("paths")
        .value_name("PATHS")
        .help("Search paths")
        .default_value(".")
        .min_values(1)
        .allow_invalid_utf8(true)
    )
    .arg(
        Arg::new("names")
        .value_name("NAME")
        .long("name")
        .short('n')
        .help("Name")
        .takes_value(true)
        .multiple_occurrences(true)
        .allow_invalid_utf8(true)
    )
    .arg(
        Arg::new("types")
        .value_name("TYPE")
        .long("type")
        .short('t')
        .help("Limit file types returned")
        .takes_value(true)
        .multiple_occurrences(true)
        .possible_values(&["d", "f", "l"])
        .allow_invalid_utf8(true)
    )
    .get_matches();
    let names = matches.values_of_lossy("names")
        .map(|vals| {
            vals.into_iter()
            .map(|name| {
                Regex::new(&name)
                    .map_err(|_| format!("Invalid --name \"{}\"", name))
            })
            .collect::<Result<Vec<_>, _>>()
        })
        .transpose()?
        .unwrap_or_default();
    let entry_types = matches
        .values_of_lossy("types")
        .map(|vals| {
            vals.iter()
                .map(|val| match val.as_str()  {
                    "d" => Dir,
                    "f" => File,
                    "l" => Link,
                    _ => unreachable!("Invalid type"),
                })
                .collect()
        })
        .unwrap_or_default();

    Ok(Config {
        paths: matches.values_of_lossy("paths").unwrap(),
        names,
        entry_types,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for path in config.paths {
        for file in WalkDir::new(path) {
            match file {
                Err(e) => eprintln!("{}", e),
                Ok(entry) => {
                    if (config.entry_types.is_empty()
                        || config.entry_types.iter().any(|entry_type| {
                            match entry_type {
                                Link => entry.file_type().is_symlink(),
                                Dir => entry.file_type().is_dir(),
                                File => entry.file_type().is_file(),
                            }
                        }))
                        && (config.names.is_empty()
                            || config.names.iter().any(|re| {
                                re.is_match(&entry.file_name().to_string_lossy())
                            })
                        )
                    {
                        println!("{}", entry.path().display());
                    }
                },
            }
        }
    }

    Ok(())
}