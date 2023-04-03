use lazy_static::lazy_static;
use regex::Regex;
use std::fs::read_to_string;

lazy_static! {
    static ref RANGE: Regex = Regex::new(r"^[A-F0-9]{4,5}\.\.[A-F0-9]{4,5}$").unwrap();
    static ref CODE_POINT: Regex = Regex::new(r"^[A-F0-9]{4,5}( [A-F0-9]{4,5})*$").unwrap();
}

fn bytes_from_codepoints(input: &str) -> Vec<String> {
    if RANGE.is_match(input) {
        let (first, last) = input.split_once("..").unwrap();
        let first = u32::from_str_radix(first, 16).unwrap();
        let last = u32::from_str_radix(last, 16).unwrap();

        (first..=last)
            .into_iter()
            .map(|x| char::from_u32(x).unwrap().to_string())
            .collect()
    } else if CODE_POINT.is_match(input) {
        let s = input
            .split(' ')
            .map(|s| u32::from_str_radix(s, 16).unwrap())
            .map(|x| char::from_u32(x).unwrap())
            .collect::<String>();
        vec![s]
    } else {
        unreachable!("Unexpected emoji codepoint: '{input}'");
    }
}

fn main() {
    let file_path = std::env::args().nth(1).expect("Usage: emoji_parser <file>");
    let entries = read_to_string(&file_path)
        .unwrap()
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
        .filter_map(|s| {
            let mut parts = s.split(';').map(|s| s.trim());
            let code_point = parts.next().unwrap();
            let type_field = parts.next().unwrap();

            if let "RGI_Emoji_Modifier_Sequence" = type_field {
                None
            } else {
                Some(code_point)
            }
        })
        .flat_map(bytes_from_codepoints)
        .collect::<Vec<_>>();

    let output = serde_json::to_string(&entries).unwrap();
    println!("{output}");
}
