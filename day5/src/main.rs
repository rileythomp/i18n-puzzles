use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut ans = 0;
    let mut grid: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        grid.push(line);
    }

    let mut cur_x = 0;
    for row in grid {
        if row.chars().nth(cur_x).unwrap() == '💩' {
            ans += 1
        }
        cur_x = (cur_x + 2) % row.chars().count();
    }

    println!("{}", ans);

    Ok(())
}
