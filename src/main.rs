use clap::{value_parser, Arg, Command};
use rand::Rng;

mod texts;
use texts::text_vars::{ITEXT, QTEXT};

fn main() {
    let res = Command::new("cohle")
        .about("Rust CLI that prints Rust Cohle quotes")
        .author("shettysach")
        .arg(
            Arg::new("index")
                .value_parser(value_parser!(usize))
                .required(false)
                .help("Index of the quote (Optional)"),
        )
        .subcommand(
            Command::new("list")
                .visible_alias("l")
                .about("Lists all the quotes along with their indices"),
        )
        .subcommand(
            Command::new("quote")
                .visible_alias("q")
                .about("Print only quote without image"),
        )
        .subcommand(
            Command::new("image")
                .visible_alias("i")
                .about("Print only image without quote"),
        )
        .get_matches();

    let quotes = QTEXT.lines().collect::<Vec<&str>>();

    let imgs = ITEXT.lines().collect::<Vec<&str>>();

    match res.subcommand_name() {
        Some("list") => {
            cohle::list_quotes(quotes);
        }
        Some("quote") => {
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
        Some("image") => {
            print!("{}", ITEXT);
        }
        _ => {
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
    }
}
