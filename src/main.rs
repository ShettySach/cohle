use clap::{value_parser, Arg, Command};
use rand::Rng;

mod texts;
use texts::text_vars::{IMARR, QTEXT};

fn main() {
    let res = Command::new("cohle")
        .about(color_print::cstr!(
        r#"<bold><underline><red>Cohle:</red></underline></bold> Rust CLI that prints Rust Cohle quotes"#
        ))
        .version("0.1.0")
        .arg(
            Arg::new("quote_index")
                .value_parser(value_parser!(usize))
                .required(false)
                .help("Index of the quote (Optional)"),
        )
        .arg(
            Arg::new("image_index")
                .value_parser(value_parser!(usize))
                .required(false)
                .help("Index of the quote (Optional)"),
        )
        .arg(
            Arg::new("colour")
                .short('c')
                .long("col")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .required(false)
                .help("Use 'cohle -q <colour>' to print quote in colour"),
        )
        .arg(
            Arg::new("background")
                .short('b')
                .long("bg")
                .action(clap::ArgAction::SetTrue)
                .help("Use '-b' to print image with black background"),
        )
        .subcommand(
            Command::new("list")
                .visible_alias("l")
                .about("Lists all the quotes along with their indices"),
        )
        .subcommand(
            Command::new("display")
                .visible_alias("d")
                .about("Displays all the images along with their indices"),
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
    let imarr = IMARR;

    let qind: usize = if let Some(value) = res.get_one::<usize>("quote_index") {
        *value
    } else {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..quotes.len())
    };

    let imind: usize = if let Some(value) = res.get_one::<usize>("image_index") {
        *value
    } else {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..imarr.len())
    };

    let blk = res.get_flag("background");

    let img = *imarr.get(imind).expect("Out of index");

    let qcol: &str = if let Some(value) = res.get_one::<String>("colour") {
        value.as_str()
    } else {
        "white"
    };

    match res.subcommand_name() {
        Some("list") => {
            cohle::list_quotes(quotes);
        }
        Some("display") => {
            cohle::display_images(imarr);
        }
        Some("quote") => {
            cohle::fill_print(quotes.get(qind).expect("Out of index"), qcol);
        }
        Some("image") => cohle::img_print(img, &blk),
        _ => {
            cohle::print_img(img, quotes[qind], qcol, &blk);
        }
    }
}
