use chrono::{TimeZone, Utc};
use chrono_tz::Tz;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut ans = 0;
    let mut line1: String = String::new();
    let mut line2: String = String::new();
    for line in reader.lines() {
        let line = line?;
        if line1 == "".to_string() {
            line1 = line;
        } else if line2 == "".to_string() {
            line2 = line;
        } else {
            // do stuff
            let parts1: Vec<String> = line1
                .split("  ")
                .map(|part| part.trim().to_string())
                .filter(|part| !part.is_empty())
                .collect();
            let parts2: Vec<String> = line2
                .split("  ")
                .map(|part| part.trim().to_string())
                .filter(|part| !part.is_empty())
                .collect();
            let format = "%b %d, %Y, %H:%M";
            let tz1: Tz = parts1.get(1).unwrap().parse().unwrap();
            let tz2: Tz = parts2.get(1).unwrap().parse().unwrap();

            let date1 = parts1.get(2).unwrap().clone();
            let date2 = parts2.get(2).unwrap();

            let local_time1 = tz1.datetime_from_str(&date1, format).unwrap();
            let local_time2 = tz2.datetime_from_str(&date2, format).unwrap();

            let utc1 = local_time1.with_timezone(&Utc);
            let utc2 = local_time2.with_timezone(&Utc);

            let dur = utc2 - utc1;
            let mins = dur.num_minutes();

            ans += mins;
            line1 = "".to_string();
            line2 = "".to_string();
        }
    }
    println!("{}", ans);

    Ok(())
}
