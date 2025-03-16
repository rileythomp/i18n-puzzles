use encoding::all::ISO_8859_1;
use encoding::{EncoderTrap, Encoding};
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct WordPattern {
    length: usize,
    letter: char,
    letter_pos: usize,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut reading_words = true;
    let mut words: Vec<String> = Vec::new();
    let mut word_patterns: Vec<WordPattern> = Vec::new();
    let mut i = 0;

    for line in reader.lines() {
        let line = line?;
        if line == "".to_string() {
            reading_words = false;
            continue;
        }

        if reading_words {
            i += 1;
            if i % 15 == 0 {
                let latin_1_bs = ISO_8859_1.encode(&line, EncoderTrap::Strict).unwrap();
                let orig = String::from_utf8(latin_1_bs).unwrap();
                let latin_1_bs2 = ISO_8859_1.encode(&orig, EncoderTrap::Strict).unwrap();
                let orig2 = String::from_utf8(latin_1_bs2).unwrap();
                words.push(orig2);
            } else if i % 3 == 0 || i % 5 == 0 {
                let latin_1_bs = ISO_8859_1.encode(&line, EncoderTrap::Strict).unwrap();
                let orig = String::from_utf8(latin_1_bs).unwrap();
                words.push(orig);
            } else {
                words.push(line);
            }
        } else {
            let mut letter = ' ';
            let mut letter_pos = 0;
            let length = line.trim().chars().count();
            for (i, c) in line.trim().chars().enumerate() {
                if c != '.' {
                    letter = c;
                    letter_pos = i;
                }
            }
            word_patterns.push(WordPattern {
                length: length,
                letter: letter,
                letter_pos: letter_pos,
            })
        }
    }

    let mut ans = 0;
    for wp in word_patterns {
        for (i, word) in words.iter().enumerate() {
            if word.chars().count() == wp.length
                && word.chars().nth(wp.letter_pos).unwrap() == wp.letter
            {
                ans += i + 1;
                break;
            }
        }
    }

    println!("{}", ans);

    Ok(())
}
