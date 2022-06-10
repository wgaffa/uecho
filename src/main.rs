use std::char::REPLACEMENT_CHARACTER;

fn main() {
    for input in std::io::stdin().lines() {
        if let Ok(input) = input {
            let words = input.split_whitespace()
                .map(|x| into_unicode(x) );

            for word in words {
                print!("{}", word);
            }
        }
    }
}

fn into_unicode(input: &str) -> char {
    let input = match input.get(0..2) {
        Some(hex) if hex == "0x" => u32::from_str_radix(&input[2..], 16),
        Some(_) => u32::from_str_radix(input, 10),
        _ => panic!("Error when reading stdin"),
    };

    let input = input.unwrap_or(REPLACEMENT_CHARACTER as u32);
    char::from_u32(input).unwrap_or(REPLACEMENT_CHARACTER)
}
