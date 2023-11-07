use clap::{value_parser, Arg, Command};
use rand::Rng;

mod texts;
use texts::text_vars::{ICOL, ITEXT, QCOL, QTEXT};

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
        .arg(
            Arg::new("icol")
                .short('i')
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .required(false)
                .help("Colour of image"),
        )
        .arg(
            Arg::new("qcol")
                .short('q')
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .required(false)
                .help("Colour of quote"),
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
    let imgs = ITEXT;

    let ind: usize = if let Some(value) = res.get_one::<usize>("index") {
        *value
    } else {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..quotes.len())
    };

    let icol: &str = if let Some(value) = res.get_one::<String>("icol") {
        value.as_str()
    } else {
        ICOL
    };

    let qcol: &str = if let Some(value) = res.get_one::<String>("qcol") {
        value.as_str()
    } else {
        QCOL
    };

    match res.subcommand_name() {
        Some("list") => {
            cohle::list_quotes(quotes);
        }
        Some("quote") => {
            cohle::fill_print(quotes[ind], qcol);
        }
        Some("image") => {
            cohle::colstr(ITEXT, icol);
        }
        _ => {
            cohle::print_img(imgs, quotes[ind], icol, qcol);
        }
    }
}
