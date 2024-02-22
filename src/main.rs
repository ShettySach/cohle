use clap::{value_parser, Arg, Command};
use colored::Colorize;

mod texts;
use texts::text_vars::{ITEXT, QTEXT};

fn main() {
    let res = Command::new("cohle")
        .about(format!(
            "{} {}",
            "Cohle:".blue().bold().underline(),
            "Rust CLI that prints Rust Cohle quotes."
        ))
        .version("0.1.0")
        .arg(
            Arg::new("quote_index")
                .value_parser(value_parser!(u8))
                .required(false)
                .help("Index of the quote [Optional]"),
        )
        .arg(
            Arg::new("colour")
                .short('c')
                .long("col")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .required(false)
                .help("Use 'cohle -c <colour>' to print quote in colour"),
        )
        .arg(
            Arg::new("background")
                .short('b')
                .long("bg")
                .action(clap::ArgAction::SetTrue)
                .help("Print image with black background [Better for lighter backgrounds]"),
        )
        .subcommand(
            Command::new("list")
                .visible_alias("l")
                .about("Lists all the quotes and colours along with their indices"),
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
    let img = ITEXT;

    let qind: u8 = if let Some(value) = res.get_one::<u8>("quote_index") {
        *value
    } else {
        fastrand::u8(0..quotes.len() as u8)
    };

    let blk = res.get_flag("background");

    let qcol: &str = if let Some(value) = res.get_one::<String>("colour") {
        value.as_str()
    } else {
        "white"
    };

    match res.subcommand_name() {
        Some("list") => {
            cohle::list_quotes(quotes);
        }
        Some("quote") => {
            cohle::only_quote(quotes.get(qind as usize).expect("Out of index"), qcol);
        }
        Some("image") => {
            cohle::only_image(img, &blk);
        }
        _ => {
            cohle::quote_image(img, quotes[qind as usize], qcol, &blk);
        }
    }
}
