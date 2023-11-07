use crossterm::style::{StyledContent, Stylize};
use crossterm::terminal;
use std::borrow::Cow;
use textwrap::{fill, wrap};

pub fn fill_print(input_text: &str, qcolr: &str) {
    let term_width = terminal::size().unwrap().0 as usize;
    let filled_text = fill(input_text, term_width);
    println!("{}", colstr(filled_text.as_str(), qcolr));
}

pub fn wrap_text(input_text: &str, width: usize) -> Vec<Cow<'_, str>> {
    let wrapped_text = wrap(input_text, width);
    wrapped_text
}

pub fn list_quotes(quotes: Vec<&str>) {
    println!("List of quotes with indices - \n");
    for (ind, quote) in quotes.iter().enumerate() {
        println!(r#"  {} - {}...""#, ind, &quote[0..35])
    }
    println!("\n Use 'cohle n' to print the nth quote or use 'cohle' to print a random quote.");
}

pub fn colstr<'a>(text: &'a str, colr: &str) -> StyledContent<&'a str> {
    match colr {
        "r" | "red" => text.red(),
        "b" | "blue" => text.blue(),
        "y" | "yellow" => text.yellow(),
        "g" | "green" => text.green(),
        "m" | "magenta" => text.magenta(),
        "c" | "cyan" => text.cyan(),
        "dr" | "dark_red" => text.dark_red(),
        "db" | "dark_blue" => text.dark_blue(),
        "dy" | "dark_yellow" => text.dark_yellow(),
        "dg" | "dark_green" => text.dark_cyan(),
        "dm" | "dark_magenta" => text.dark_magenta(),
        "dc" | "dark_cyan" => text.dark_cyan(),
        _ => text.white(),
    }
}

pub fn colcow<'a>(text: &'a Cow<'_, str>, colr: &str) -> StyledContent<&'a str> {
    match colr {
        "r" | "red" => text.red(),
        "b" | "blue" => text.blue(),
        "y" | "yellow" => text.yellow(),
        "g" | "green" => text.green(),
        "m" | "magenta" => text.magenta(),
        "c" | "cyan" => text.cyan(),
        "dr" | "dark_red" => text.dark_red(),
        "db" | "dark_blue" => text.dark_blue(),
        "dy" | "dark_yellow" => text.dark_yellow(),
        "dg" | "dark_green" => text.dark_cyan(),
        "dm" | "dark_magenta" => text.dark_magenta(),
        "dc" | "dark_cyan" => text.dark_cyan(),
        _ => text.white(),
    }
}

pub fn print_img(img: Vec<&str>, quote: &str, icolr: &str, qcolr: &str) {
    let term_width = terminal::size().unwrap().0 as usize;
    let width = term_width.checked_sub(img[0].len() + 4);

    match width {
        Some(value) => {
            let imlen = img.len();
            let wquote = wrap_text(quote, value);
            let qlen = wquote.len();
            let start = imlen.abs_diff(qlen) / 2 as usize;

            if imlen >= qlen {
                for i in 0..start {
                    println!("{}", colstr(img[i], icolr));
                }
                for i in start..(start + qlen) {
                    println!(
                        "{}  {}",
                        colstr(img[i], icolr),
                        colcow(&wquote[i - start], qcolr)
                    );
                }

                for i in (start + qlen)..imlen {
                    println!("{}", colstr(img[i], icolr));
                }
            } else {
                fill_print(quote, qcolr);
                println!("\n{}",
                    "Terminal width too small to print message, try expanding window or reducing font size".dark_grey()
                );
            }
        }
        _ => {
            fill_print(quote, qcolr);
            println!("\n{}",
                    "Terminal width too small to print message, try expanding window or reducing font size".dark_grey()
                );
        }
    }
}
