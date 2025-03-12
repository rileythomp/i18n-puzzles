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
        let num_bytes = line.len();
        let num_chars = line.chars().count();
        if num_chars <= 140 && num_bytes <= 160 {
            ans += 13;
        } else if num_chars <= 140 {
            ans += 7;
        } else if num_bytes <= 160 {
            ans += 11;
        }
    }
    println!("{}", ans);

    Ok(())
}
