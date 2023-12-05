use crossterm::style::{StyledContent, Stylize};
use crossterm::terminal;
use textwrap::{fill, wrap};

pub fn only_quote(input_text: &str, qcolr: &str) {
    let term_width = terminal::size().unwrap().0 as usize;
    let filled_text = fill(input_text, term_width);
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
    dr | darkred
    db | darkblue
    dy | darkyellow
    dg | darkgreen
    dm | darkmagenta
    dc | darkcyan
    bk | black
        ";
    println!("\nColours - \n {}", cols.blue());

    println!("\n Use 'cohle -q <colour>' to print quote in colour");
}

pub fn colstr<'a>(text: &'a str, colr: &str) -> StyledContent<&'a str> {
    match colr {
        "r" | "red" => text.red(),
        "b" | "blue" => text.blue(),
        "y" | "yellow" => text.yellow(),
        "g" | "green" => text.green(),
        "m" | "magenta" => text.magenta(),
        "c" | "cyan" => text.cyan(),
        "dr" | "darkred" => text.dark_red(),
        "db" | "darkblue" => text.dark_blue(),
        "dy" | "darkyellow" => text.dark_yellow(),
        "dg" | "darkgreen" => text.dark_cyan(),
        "dm" | "darkmagenta" => text.dark_magenta(),
        "dc" | "darkcyan" => text.dark_cyan(),
        "bk" | "black" => text.black(),
        _ => text.white(),
    }
}

pub fn quote_image(img: &str, quote: &str, qcolr: &str, blk: &bool) {
    let term_width = terminal::size().unwrap().0 as usize;
    let width = term_width.checked_sub(56);

    let bg = if *blk { "[107;40m" } else { "" };

    match width {
        Some(value) => {
            let imvec = img.lines().collect::<Vec<&str>>();
            let imlen = imvec.len();
            let qvec = wrap(quote, value);
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
                "Terminal width too small to print image. \n Resize or reduce font".grey()
            );
            only_quote(quote, qcolr);
        }
    }
}
