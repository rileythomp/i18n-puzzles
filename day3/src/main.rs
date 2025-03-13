use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

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
        let mut has_upper = false;
        let mut has_lower = false;
        let mut has_non_ascii = false;
        let mut has_num = false;
        for c in line.chars() {
            if c.is_uppercase() {
                has_upper = true;
            }
            if c.is_lowercase() {
                has_lower = true;
            }
            if c.is_digit(10) {
                has_num = true;
            }
            if !c.is_ascii() {
                has_non_ascii = true;
            }
        }

        if valid_len && has_upper && has_lower && has_non_ascii && has_num {
            ans += 1
        }
    }
    println!("{}", ans);

    Ok(())
}
