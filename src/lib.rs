use crossterm::style::{StyledContent, Stylize};
use crossterm::terminal;
use std::borrow::Cow;
use textwrap::{fill, wrap};

pub fn fill_text(input_text: &str) -> String {
    let term_width = terminal::size().unwrap().0 as usize;
    let filled_text = fill(input_text, term_width);
    filled_text
}

pub fn wrap_text(input_text: &str, width: usize) -> Vec<Cow<'_, str>> {
    let wrapped_text = wrap(input_text, width);
    wrapped_text
}

pub fn colprint<'a>(text: &'a str, colr: &str) -> StyledContent<&'a str> {
    match colr {
        "r" | "red" => text.red(),
        "b" | "blue" => text.blue(),
        "y" | "yellow" => text.yellow(),
        "g" | "green" => text.green(),
        "m" | "magenta" => text.magenta(),
        "c" | "cyan" => text.cyan(),
        _ => text.white(),
    }
}

pub fn print_img(img: Vec<&str>, quote: &str) {
    let term_width = terminal::size().unwrap().0 as usize;
    let width = term_width.checked_sub(img[0].len() + 6);

    match width {
        Some(value) => {
            let imlen = img.len();
            let wquote = wrap_text(quote, value);
            let qlen = wquote.len();
            let start = imlen.abs_diff(qlen) / 2 as usize;

            if imlen >= qlen {
                for i in 0..start {
                    println!("{}", img[i]);
                }
                for i in start..(start + qlen) {
                    println!("{}  {}", img[i], wquote[i - start]);
                }

                for i in (start + qlen)..imlen {
                    println!("{}", img[i]);
                }
            } else {
                println!("{}", fill_text(quote));
                println!("\n{}",
                    "Terminal width too small to print message, try expanding window or reducing font size".dark_grey()
                );
            }
        }
        _ => {
            println!("{}", fill_text(quote));
            println!("\n{}",
                    "Terminal width too small to print message, try expanding window or reducing font size".dark_grey()
                );
        }
    }
}
