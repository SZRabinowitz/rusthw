use clap::{Parser, ValueEnum};
use colored::*;

#[derive(Parser)]
struct Args {
    /// Color for 'Hello' (default: Green)
    #[arg(short = 'H', long, value_enum, default_value = "green")]
    hello_color: ColorChoice,

    /// Color for 'World' (default: Blue)
    #[arg(short = 'W', long, value_enum, default_value = "blue")]
    world_color: ColorChoice,

    /// Add an exclamation mark (default: false)
    #[arg(short, long)]
    exclamation: bool,

    /// Specify casing style (default: Capitalize)
    #[arg(short = 'C', long, value_enum, default_value = "capitalize")]
    casing: Casing,
}

#[derive(ValueEnum, Clone)]
enum ColorChoice {
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Cyan,
    White,
    Black,
}

#[derive(ValueEnum, Clone)]
enum Casing {
    Uppercase,
    Capitalize,
    Lowercase,
}

fn main() {
    let args = Args::parse();

    let hello = format_hello(&args);
    let world = format_world(&args);

    print_message(&hello, &world, args.exclamation);
}

fn format_hello(args: &Args) -> String {
    match args.casing {
        Casing::Uppercase => "HELLO".to_string(),
        Casing::Capitalize => "Hello".to_string(),
        Casing::Lowercase => "hello".to_string(),
    }
    .color(match args.hello_color {
        ColorChoice::Red => Color::Red,
        ColorChoice::Green => Color::Green,
        ColorChoice::Blue => Color::Blue,
        ColorChoice::Yellow => Color::Yellow,
        ColorChoice::Magenta => Color::Magenta,
        ColorChoice::Cyan => Color::Cyan,
        ColorChoice::White => Color::White,
        ColorChoice::Black => Color::Black,
    })
    .to_string()
}

fn format_world(args: &Args) -> String {
    match args.casing {
        Casing::Uppercase => "WORLD".to_string(),
        Casing::Capitalize => "World".to_string(),
        Casing::Lowercase => "world".to_string(),
    }
    .color(match args.world_color {
        ColorChoice::Red => Color::Red,
        ColorChoice::Green => Color::Green,
        ColorChoice::Blue => Color::Blue,
        ColorChoice::Yellow => Color::Yellow,
        ColorChoice::Magenta => Color::Magenta,
        ColorChoice::Cyan => Color::Cyan,
        ColorChoice::White => Color::White,
        ColorChoice::Black => Color::Black,
    })
    .to_string()
}

fn print_message(hello: &str, world: &str, exclamation: bool) {
    if exclamation {
        println!("{} {}!", hello, world);
    } else {
        println!("{} {}", hello, world);
    }
}
