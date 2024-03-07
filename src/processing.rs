use colored::{ColoredString, Colorize};
use terminal_size::{terminal_size, Width};
use textwrap::{fill, wrap};

pub fn only_quote(input_text: &str, qcolr: &str) {
    let (Width(term_width), _) = terminal_size().unwrap();
    let filled_text = fill(input_text, term_width as usize);
    println!("{}", colstr(filled_text.as_str(), qcolr));
}

pub fn only_image(img: &str, blk: &bool) {
    if *blk {
        let imvec = img.lines().collect::<Vec<&str>>();
        imvec.iter().for_each(|imline| {
            println!("[107;40m{}", imline);
        })
    } else {
        println!("{}", img);
    };
}

pub fn list_quotes(quotes: Vec<&str>) {
    println!("Quotes and indices - \n");

    quotes.iter().enumerate().for_each(|(ind, quote)| {
        let n = quote.len();
        let part = if n < 45 {
            &quote[1..n - 2]
        } else {
            &quote[1..45]
        };
        println!(r#"  {}) {}..."#, ind, &part.blue())
    });

    println!("\n Use 'cohle n' to print the nth quote or use 'cohle' to print a random quote.");

    let cols = "
    r | red
    b | blue
    y | yellow
    g | green
    m | magenta
    c | cyan
    bk | black
    lr | bright_red
    lg | bright_green
    lb | bright_blue
    ly | bright_yellow
    lm | bright_magenta
    lc | bright_cyan
    lbk | bright_black
    lw | bright_white";
    println!("\nColours - \n {}", cols.bright_blue());

    println!("\n Use 'cohle -q <colour>' to print quote in colour");
}

pub fn colstr<'a>(text: &'a str, colr: &str) -> ColoredString {
    match colr {
        "r" | "red" => text.red(),
        "g" | "green" => text.green(),
        "b" | "blue" => text.blue(),
        "y" | "yellow" => text.yellow(),
        "m" | "magenta" => text.magenta(),
        "c" | "cyan" => text.cyan(),
        "bk" | "black" => text.black(),
        "lr" | "bright_red" => text.bright_red(),
        "lg" | "bright_green" => text.bright_green(),
        "lb" | "bright_blue" => text.bright_blue(),
        "ly" | "bright_yellow" => text.bright_yellow(),
        "lm" | "bright_magenta" => text.bright_magenta(),
        "lc" | "bright_cyan" => text.bright_cyan(),
        "lbk" | "bright_black" => text.bright_black(),
        "lw" | "bright_white" => text.bright_white(),
        _ => text.white(),
    }
}

pub fn quote_image(img: &str, quote: &str, qcolr: &str, blk: &bool) {
    let (Width(term_width), _) = terminal_size().unwrap();
    let width = term_width.checked_sub(56);

    let bg = if *blk { "[107;40m" } else { "" };

    match width {
        Some(value) => {
            let imvec = img.lines().collect::<Vec<&str>>();
            let imlen = imvec.len();
            let qvec = wrap(quote, value as usize);
            let qlen = qvec.len();
            let start = imlen.abs_diff(qlen) / 2 as usize;

            if imlen >= qlen {
                for i in 0..start {
                    println!("{}{}", bg, imvec[i]);
                }
                for i in start..(start + qlen) {
                    println!("{}{} {}", bg, imvec[i], colstr(&qvec[i - start], qcolr));
                }

                for i in (start + qlen)..imlen {
                    println!("{}{}", bg, imvec[i]);
                }
            } else {
                only_image(img, blk);
                println!();
                only_quote(quote, qcolr);
            }
        }
        _ => {
            println!(
                "{}",
                "Terminal width too small to print image. \n Resize terminal / reduce font or use 'cohle q'".bright_black()
            );
            only_quote(quote, qcolr);
        }
    }
}
