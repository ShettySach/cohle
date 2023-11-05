use clap::{command, value_parser, Arg, Command};
use rand::Rng;
use std::fs;

fn main() {
    let res = command!()
        .arg(
            Arg::new("index")
                .value_parser(value_parser!(usize))
                .required(false)
                .help("Index of the quotes"),
        )
        .subcommand(Command::new("list").about("Lists quotes"))
        .get_matches();

    let content = fs::read_to_string("quotes.txt").expect("Error in reading quotes.txt");
    let parts = content.lines();

    let quotes = parts.collect::<Vec<&str>>();

    match res.subcommand_name() {
        Some("list") => {
            println!("List of quotes with indices - \n");
            for (ind, quote) in quotes.iter().enumerate() {
                println!("{} - \t{}...", ind, &quote[0..30])
            }
            println!("Use - cohle n - to print the nth quote");
        }
        _ => {
            if res.contains_id("index") {
                let ind: usize = *res
                    .get_one("index")
                    .expect("Index must have value from 0 to n");
                println!("{}", quotes[ind]);
            } else {
                let mut rng = rand::thread_rng();
                let ind: usize = rng.gen_range(0..quotes.len());
                println!("{}", quotes[ind]);
            }
        }
    }
}
