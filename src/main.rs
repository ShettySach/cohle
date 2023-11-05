use clap::{command, value_parser, Arg};
use std::fs;

fn main() {
    let res = command!()
        .arg(
            Arg::new("index")
                .value_parser(value_parser!(usize))
                .required(false)
                .help("Index of the quotes"),
        )
        .arg(
            Arg::new("list")
                .short('l')
                .long("list")
                .required(false)
                .help("List all quotes")
                .conflicts_with("index"),
        )
        .get_matches();

    let content = fs::read_to_string("quotes.txt").expect("Error in reading quotes.txt");
    let parts = content.lines();

    let quotes = parts.collect::<Vec<&str>>();

    if res.contains_id("index") {
        let ind: usize = *res
            .get_one("index")
            .expect("Index must have value from 0 to n");
        println!("{}", quotes[ind]);
    } else {
        print!("Hello");
    }
}
