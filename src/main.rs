#![doc = include_str!("../README.md")]

use std::char::REPLACEMENT_CHARACTER;
use std::io::BufRead;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "Echo the unicode character")]
struct Opts {
    #[structopt(short = "e", long = "echo-original")]
    /// Echo original input if it fails to find it's unicode character
    echo_orig: bool,

    /// A list of unicode points to convert
    codes: Vec<String>,
}

fn main() {
    let opts = Opts::from_args();

    let keep_orig = |def: &str| {
        if opts.echo_orig {
            String::from(def)
        } else {
            String::from(REPLACEMENT_CHARACTER)
        }
    };
    let get_code = |x: &str| into_unicode(x).map(String::from).unwrap_or_else(|| keep_orig(x));

    if opts.codes.is_empty() {
        for input in std::io::stdin().lock().lines().flatten() {
            let output = itertools::Itertools::intersperse(
                input.split_whitespace().map(get_code),
                String::from(" "),
            )
            .collect::<String>();

            print!("{}", output);
        }
    } else {
        let output = itertools::Itertools::intersperse(
            opts.codes.into_iter().map(|x| get_code(&x)),
            String::from(" "),
        )
        .collect::<String>();

        print!("{}", output);
    }
}

fn into_unicode(input: &str) -> Option<char> {
    let input = match input {
        x if x.starts_with("0x") => u32::from_str_radix(&input[2..], 16).ok()?,
        x => x.parse().ok()?,
    };

    char::from_u32(input)
}
