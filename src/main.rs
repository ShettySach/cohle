use clap::{value_parser, Arg, Command};
use rand::Rng;
use std::fs;

fn main() {
    let res = Command::new("cohle")
        .about("Rust CLI that prints Rust Cohle quotes")
        .author("shettysach")
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
        .subcommand(
            Command::new("image")
                .visible_alias("i")
                .about("Print with image"),
        )
        .get_matches();

    let content = fs::read_to_string("quotes.txt").expect("Error in reading quotes.txt");
    let quotes = content.lines().collect::<Vec<&str>>();

    let content = fs::read_to_string("imgs/0.txt").expect("Error in reading ascii_img.txt");
    let imgs = content.lines().collect::<Vec<&str>>();

    match res.subcommand_name() {
        Some("list") => {
            cohle::list_quotes(quotes);
        }
        Some("image") => {
            if res.contains_id("index") {
                let ind: usize = *res
                    .get_one("index")
                    .expect("Index must have value from 0 to n");
                cohle::print_img(imgs, quotes[ind]);
            } else {
                let mut rng = rand::thread_rng();
                let ind: usize = rng.gen_range(0..quotes.len());
                cohle::print_img(imgs, quotes[ind]);
            }
        }
        _ => {
            if res.contains_id("index") {
                let ind: usize = *res
                    .get_one("index")
                    .expect("Index must have value from 0 to n");
                cohle::fill_print(quotes[ind]);
            } else {
                let mut rng = rand::thread_rng();
                let ind: usize = rng.gen_range(0..quotes.len());
                cohle::fill_print(quotes[ind]);
            }
        }
    }
}
