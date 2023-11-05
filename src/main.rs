use clap::{command, value_parser, Arg, Command};
use rand::Rng;
use std::fs;

fn main() {
    let res = command!()
        .arg(
            Arg::new("index")
                .value_parser(value_parser!(usize))
                .required(false)
                .help("Index of the quotes (Optional)"),
        )
        .subcommand(
            Command::new("list")
                .visible_alias("l")
                .about("Lists all the quotes along with their indices"),
        )
        .get_matches();

    let content = fs::read_to_string("quotes.txt").expect("Error in reading quotes.txt");
    let parts = content.lines();
    let quotes = parts.collect::<Vec<&str>>();

    match res.subcommand_name() {
        Some("list") => {
            println!("List of quotes with indices - \n");
            for (ind, quote) in quotes.iter().enumerate() {
                println!(r#"  {} - {}...""#, ind, &quote[0..35])
            }
            println!(
                "\n Use 'cohle n' to print the nth quote or use 'cohle' to print a random quote."
            );
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
