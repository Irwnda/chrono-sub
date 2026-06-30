use crossterm::style::{Color, Stylize};
use std::{env, process};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn print_version() {
    println!("{} {}", "chrono-sub".cyan().bold(), VERSION.dark_grey());
}

fn print_help() {
    println!("{} {}", "chrono-sub".cyan().bold(), VERSION.dark_grey());
    println!();
    println!("An intuitive CLI tool for synchronizing subtitle files with millisecond precision");
    println!();

    println!("{}", "USAGE:".yellow().bold());
    println!("    chrono-sub [OPTIONS]");
    println!();

    println!("{}", "OPTIONS:".yellow().bold());
    println!("    {} Print help information", "-h, --help".green());
    println!("    {} Print version information", "-V, --version".green());
    println!();

    println!(
        "{}",
        "Run 'chrono-sub' without arguments to start the interactive interface."
            .italic()
            .dark_grey()
    );
}

pub fn parse() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "--version" | "-V" => {
                print_version();
                process::exit(0);
            }
            "--help" | "-h" => {
                print_help();
                process::exit(0);
            }
            _ => {
                eprintln!(
                    "{}: Unknown argument '{}'",
                    "error".with(Color::Red).bold(),
                    args[1]
                );
                eprintln!("Run '{}' for usage information", "chrono-sub --help".cyan());
                process::exit(1);
            }
        }
    }
}
