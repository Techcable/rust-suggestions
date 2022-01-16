//! Minimal command line interface to the `suggestions` library
//!
//! Has zero additional dependencies.

use std::{env, io, process};

#[derive(Default, Debug)]
pub struct Flags {
    single_suggestion: bool,
    quote_output: bool,
    json: bool,
    require_suggestions: bool,
}

const HELP: &str = r##"suggestions
A minimal CLI wrapping the 'suggestions' library.
Source: https://github.com/Techcable/rust-suggestions

Accepts "possible values" from standard input.

USAGE:
    suggestions [options] <target>...

OPTIONS:
    -s, --single             Return only a single suggestion for each target
    -q, --quote              Quote each ouptut suggestion
    --json                   Output information as valid json
    --required               Exit with an error if any of the targets have no suggestions
    -h, --help               Print this help message
"##;

pub fn main() {
    let mut args = env::args().skip(1).peekable();
    let mut flags = Flags::default();
    while args.peek().map_or(false, |arg| arg.starts_with('-')) {
        let s = args.peek().unwrap().clone();
        match s.as_str() {
            "-s" | "--single" => {
                flags.single_suggestion = true;
            }
            "-q" | "--quote" => {
                flags.quote_output = true;
            }
            "-h" | "--help" => {
                println!("{}", HELP);
                process::exit(0);
            }
            "--json" => {
                flags.json = true;
            }
            "--required" => {
                flags.require_suggestions = true;
            }
            "--" => {
                args.next().unwrap();
                break;
            }
            _ => {
                eprintln!("Unexpected flag {:?}", s);
                eprintln!("See `--help` for possible options");
                process::exit(1);
            }
        }
        // fallthrough and consume
        assert_eq!(args.next().unwrap(), s);
    }
    let targets = args.collect::<Vec<String>>();
    if targets.is_empty() {
        eprintln!("Must specify at least one target");
        process::exit(1);
    }
    let input = io::stdin();
    let mut possible_values: Vec<String> = Vec::new();
    let mut buf = String::new();
    loop {
        match input.read_line(&mut buf) {
            Ok(0) => break,
            Ok(_) => {
                possible_values.push(buf.trim_end_matches(&['\r', '\n']).into());
                buf.clear();
            }
            Err(e) => {
                eprintln!("Unexpected error reading input: {}", e);
                process::exit(1);
            }
        }
    }
    if flags.json {
        println!("{{");
    }
    let mut first_target = true;
    for target in &targets {
        if !first_target && flags.json {
            println!(",")
        }
        let mut suggestions = suggestions::provide_suggestions(target, possible_values.iter());
        if flags.single_suggestion && suggestions.len() > 1 {
            suggestions.truncate(1);
        }
        if suggestions.is_empty() && flags.require_suggestions {
            eprintln!("No relavent suggestions for {:?}", target);
            process::exit(7); // Chosen arbitrarily to be different from 1
        }
        let mut first = true;
        if flags.json {
            print!("  \"{}\":[", target.escape_default())
        }
        for mut suggestion in suggestions {
            if !first {
                if flags.json {
                    print!(",");
                } else {
                    print!(" ");
                }
            }
            if flags.quote_output || flags.json {
                suggestion = format!("\"{}\"", suggestion.escape_default());
            }
            print!("{}", suggestion);
            first = false;
        }
        if flags.json {
            print!("]");
        } else {
            println!();
        }
        first_target = false;
    }
    if flags.json {
        println!();
        println!("}}");
    }
}
