use crossterm::style::{StyledContent, Stylize};
use crossterm::terminal;
use textwrap::{fill, wrap};

pub fn fill_print(input_text: &str, qcolr: &str) {
    let term_width = terminal::size().unwrap().0 as usize;
    let filled_text = fill(input_text, term_width);
    println!("{}\n", colstr(filled_text.as_str(), qcolr));
}

pub fn img_print(img: &str, blk: &bool) {
    let (b, e) = if *blk {
        ("[107;40m", "[0m")
    } else {
        ("", "")
    };
    println!("{}{}{}\n", b, img, e);
}

pub fn list_quotes(quotes: Vec<&str>) {
    println!("List of quotes with indices - \n");
    for (ind, quote) in quotes.iter().enumerate() {
        let n = quote.len();
        let part = if n < 45 {
            &quote[1..n - 2]
        } else {
            &quote[1..45]
        };
        println!(r#"  {}) {}..."#, ind, &part.blue())
    }
    println!("\n Use 'cohle n' to print the nth quote or use 'cohle' to print a random quote.");

    let cols = "
    r | red
    b | blue
    y | yellow
    g | green
    m | magenta
    c | cyan
    dr | dark_red
    db | dark_blue
    dy | dark_yellow
    dg | dark_green
    dm | dark_magenta
    dc | dark_cyan
    bk | black
        ";
    println!("\nColours - \n {}", cols.blue());

    println!("\n Use 'cohle -q <colour>' to print quote in colour");
}

pub fn display_images(imarr: [&str; 4]) {
    println!("List of images with indices - \n");
    for (ind, image) in imarr.iter().enumerate() {
        println!("{})", ind);
        println!("{}", image);
        println!();
    }
    println!("\n Use 'cohle [quote_index] n' to print the nth image or use 'cohle' to print a random image.");
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
        "bk" | "black" => text.black(),
        _ => text.white(),
    }
}

pub fn print_img(img: &str, quote: &str, qcolr: &str, blk: &bool) {
    let term_width = terminal::size().unwrap().0 as usize;
    let width = term_width.checked_sub(56);

    let b = if *blk { "[107;40m" } else { "" };

    match width {
        Some(value) => {
            let imvec = img.lines().collect::<Vec<&str>>();
            let imlen = imvec.len();
            let qvec = wrap(quote, value);
            let qlen = qvec.len();
            let start = imlen.abs_diff(qlen) / 2 as usize;

            if imlen >= qlen {
                for i in 0..start {
                    println!("{}{}", b, imvec[i]);
                }
                for i in start..(start + qlen) {
                    println!("{}{} {}", b, imvec[i], colstr(&qvec[i - start], qcolr));
                }

                for i in (start + qlen)..imlen {
                    println!("{}{}", b, imvec[i]);
                }

                println!();
            } else {
                img_print(img, blk);
                fill_print(quote, qcolr);
            }
        }
        _ => {
            println!(
                "{}",
                "Terminal width too small to print image. \n Resize or reduce font".grey()
            );
            fill_print(quote, qcolr);
        }
    }
}
