use std::char::REPLACEMENT_CHARACTER;

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        for input in std::io::stdin().lines() {
            if let Ok(input) = input {
                let words = input.split_whitespace()
                    .map(|x| into_unicode(x) );

                for word in words {
                    print!("{}", word);
                }
            }
        }
    } else {
        for input in args {
            print!("{}", into_unicode(&input));
        }
    }
}

fn into_unicode(input: &str) -> char {
    let input = match input {
        x if x.starts_with("0x") => u32::from_str_radix(&input[2..], 16),
        x => u32::from_str_radix(&x, 10),
    };

    let input = input.unwrap_or(REPLACEMENT_CHARACTER as u32);
    char::from_u32(input).unwrap()
}
