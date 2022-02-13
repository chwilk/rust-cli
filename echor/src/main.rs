use clap::{App, Arg};

fn main() {
    let matches = App::new("echor")
    .version("0.1.0")
    .author("Chandler Wilkerson <chwilk@gmail.com>")
    .about("Rust echo")
    .arg(
        Arg::with_name("text")
        .value_name("TEXT")
        .help("Input text")
        .required(false)
        .min_values(0),
    )
    .arg(
        Arg::with_name("omit_newline")
        .short("n")
        .help("Do not print newline")
        .takes_value(false),
    )
    .get_matches();

    let omit_newline = matches.is_present("omit_newline");
    let text = match matches.values_of_lossy("text") {
        Some(i) => i,
        None => Vec::new(),
    };

    print!("{}{}", text.join(" "), if omit_newline {""} else {"\n"});
}