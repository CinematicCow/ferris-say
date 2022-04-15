use ansi_term::{self, Color};
use clap::{ArgEnum, Parser};

#[derive(Parser, Debug)]
#[clap(author, version, about = "cowsay rusty version")]
struct Args {
    /// Quote ferris will say
    #[clap(short, long)]
    quote: String,
    /// Colors to choose
    #[clap(arg_enum, short, long)]
    color: Colors,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Debug)]
enum Colors {
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
    White,
}

fn input() -> (String, Color) {
    let args = Args::parse();
    // to access quote -> args.quote;
    // to access color -> args.color

    let color_matched = match args.color {
        Colors::Red => Color::Red,
        Colors::White => Color::White,
        Colors::Cyan => Color::Cyan,
        Colors::Purple => Color::Purple,
        Colors::Blue => Color::Blue,
        Colors::Yellow => Color::Yellow,
        Colors::Green => Color::Green,
    };

    // println!("The user picked {:?}", color_matched);
    (args.quote, color_matched)
}

fn draw(quote: &str, color: &Color) {
    const FERRIS: &'static str = r"
    .
     .
      .
       █ █           █ █
        ▀█  ▄█████▄  █▀
         ▀▄███▀█▀███▄▀ 
         ▄▀███▀▀▀███▀▄ 
         █ ▄▀▀▀▀▀▀▀▄ █
    ";
    println!("{}", format!("\"{}\"{}", quote, color.paint(FERRIS)));
}

fn main() {
    let (q, c) = input();
    draw(&q, &c)
}
