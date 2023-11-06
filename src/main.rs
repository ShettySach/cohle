use clap::{value_parser, Arg, Command};
use crossterm::style::Stylize;
use crossterm::terminal;
use rand::Rng;
use std::fs;

fn print_img(img: Vec<&str>, quote: &str) {
    let term_width = terminal::size().unwrap().0 as usize;
    let width = term_width.checked_sub(img[0].len() + 6);
    match width {
        Some(value) => {
            let imlen = img.len();
            let wquote = cohle::wrap_text(quote, value);
            let qlen = wquote.len();
            let start = imlen.abs_diff(qlen) / 2 as usize;

            if imlen >= qlen {
                for i in 0..start {
                    println!("{}", img[i]);
                }
                for i in start..(start + qlen) {
                    println!("{}  {}", img[i], wquote[i - start].red());
                }

                for i in (start + qlen)..imlen {
                    println!("{}", img[i]);
                }
            } else {
                println!("{}", cohle::fill_text(quote, term_width).red());
                println!(
                    "< Terminal width too small to display image, try expanding window or reducing font size >"
                );
            }
        }
        _ => {
            println!("{}", cohle::fill_text(quote, term_width).red());
            println!("< Terminal width too small to display image, try expanding window or reducing font size >");
        }
    }
}

fn main() {
    let res = Command::new("cohle")
        .about("Rust CLI that prints Rust Cohle quotes")
        .author("github.com/shettysach")
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

    let content = fs::read_to_string("img.txt").expect("Error in reading quotes.txt");
    let imgs = content.lines().collect::<Vec<&str>>();

    let term_width = terminal::size().unwrap().0 as usize;

    match res.subcommand_name() {
        Some("list") => {
            println!("List of quotes with indices - \n");
            for (ind, quote) in quotes.iter().enumerate() {
                println!(r#"  {} - {}...""#, ind, &quote[0..35].red())
            }
            println!(
                "\n Use 'cohle n' to print the nth quote or use 'cohle' to print a random quote."
            );
        }
        Some("image") => {
            if res.contains_id("index") {
                let ind: usize = *res
                    .get_one("index")
                    .expect("Index must have value from 0 to n");
                print_img(imgs, quotes[ind]);
            } else {
                let mut rng = rand::thread_rng();
                let ind: usize = rng.gen_range(0..quotes.len());
                print_img(imgs, quotes[ind]);
            }
        }
        _ => {
            if res.contains_id("index") {
                let ind: usize = *res
                    .get_one("index")
                    .expect("Index must have value from 0 to n");
                println!("{}", cohle::fill_text(quotes[ind], term_width).red());
            } else {
                let mut rng = rand::thread_rng();
                let ind: usize = rng.gen_range(0..quotes.len());
                println!("{}", cohle::fill_text(quotes[ind], term_width).red());
            }
        }
    }
}
