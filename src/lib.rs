use crossterm::style::Stylize;
use crossterm::terminal;
use textwrap::{fill, wrap};

pub fn fill_text(input_text: &str, term_width: usize) -> String {
    let filled_text = fill(input_text, term_width);
    filled_text
}

pub fn wrap_text(input_text: &str, term_width: usize) -> Vec<String> {
    let wrapped_text = wrap(input_text, term_width);
    let vec_str: Vec<String> = wrapped_text.iter().map(|cow| cow.to_string()).collect();
    vec_str
}

pub fn print_img(img: Vec<&str>, quote: &str) {
    let term_width = terminal::size().unwrap().0 as usize;
    let width = term_width.checked_sub(img[0].len() + 6);
    match width {
        Some(value) => {
            let imlen = img.len();
            let wquote = wrap_text(quote, value);
            let qlen = wquote.len();
            let start = (imlen - qlen) / 2 as usize;

            for i in 0..start {
                println!("{}", img[i]);
            }
            for i in start..(start + qlen) {
                println!("{}  {}", img[i], wquote[i - start].clone().red());
            }

            for i in (start + qlen)..imlen {
                println!("{}", img[i]);
            }
        }
        _ => {
            println!("{}", fill_text(quote, term_width).red());
            println!("< Terminal width too small, try expanding window or reducing font size >");
        }
    }
}
