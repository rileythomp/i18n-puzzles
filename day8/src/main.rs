use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use unicode_normalization::char::decompose_canonical;

fn remove_accent(c: char) -> char {
    let mut base = None;

    // Decomposes character into base + diacritics, keeping only the base
    decompose_canonical(c, |d| {
        if base.is_none() && d.is_alphabetic() {
            base = Some(d);
        }
    });

    base.unwrap_or(c).to_lowercase().next().unwrap_or(c)
}

fn is_vowel(c: char) -> bool {
    matches!(remove_accent(c), 'a' | 'e' | 'i' | 'o' | 'u')
}

fn is_consonant(c: char) -> bool {
    c.is_alphabetic() && !is_vowel(c)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut ans = 0;
    for line in reader.lines() {
        let line = line?;
        let str_len = line.chars().count();
        let valid_len = str_len >= 4 && str_len <= 12;
        let mut has_digit = false;
        let mut has_vowel = false;
        let mut has_consonant = false;
        let mut no_recurring = true;
        let mut seen_map: HashMap<char, bool> = HashMap::new();
        for c in line.chars() {
            if c.is_digit(10) {
                has_digit = true;
            }
            if is_vowel(c) {
                has_vowel = true;
            }
            if is_consonant(c) {
                has_consonant = true;
            }
            let normalized_char = remove_accent(c);
            if seen_map.contains_key(&normalized_char) {
                no_recurring = false;
                break;
            }
            seen_map.insert(normalized_char, true);
        }
        if valid_len && has_digit && has_vowel && has_consonant && no_recurring {
            ans += 1;
        }
    }
    println!("{}", ans);

    Ok(())
}
