use chrono::{DateTime, FixedOffset, Utc};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut utc_times: HashMap<DateTime<Utc>, i32> = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        let datetime_str = line;
        let parsed_dt: DateTime<FixedOffset> =
            datetime_str.parse().expect("Invalid datetime format");

        let utc_dt = parsed_dt.with_timezone(&Utc);

        let count = utc_times.entry(utc_dt).or_insert(0);
        if *count == 3 {
            println!("{:?}", utc_dt);
            return Ok(());
        }
        *count += 1
    }
    println!("{:?}", utc_times);

    Ok(())
}
