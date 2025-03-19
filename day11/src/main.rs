use std::collections::HashMap;
use std::fs;

fn main() {
    let filename = std::env::args().nth(1).expect("Please provide a filename.");
    let data = fs::read_to_string(filename).expect("Unable to read file.");
    let lines: Vec<&str> = data.trim().split('\n').collect();

    let odysseus = vec![
        "Οδυσσευς",
        "Odysseus",
        "Οδυσσεως",
        "Odysseos",
        "Οδυσσει",
        "Odyssei",
        "Οδυσσεα",
        "Odyssea",
        "Οδυσσευ",
        "Odysseu",
    ];

    let uppercases = "ΑΒΓΔΕΖΗΘΙΚΛΜΝΞΟΠΡΣΤΥΦΧΨΩ";
    let lowercases = "αβγδεζηθικλμνξοπρστυφχψω";

    // Create mapping for uppercase and lowercase shifts
    let mut m: HashMap<(char, usize), char> = HashMap::new();

    for (idx, uc) in uppercases.chars().enumerate() {
        for shift in 0..uppercases.len() {
            let new_char = uppercases
                .chars()
                .cycle()
                .nth((idx + shift) % uppercases.len())
                .unwrap();
            m.insert((uc, shift), new_char);
        }
    }

    for (idx, lc) in lowercases.chars().enumerate() {
        for shift in 0..lowercases.len() {
            let new_char = lowercases
                .chars()
                .cycle()
                .nth((idx + shift) % lowercases.len())
                .unwrap();
            m.insert((lc, shift), new_char);
        }
    }

    fn shift_char(c: char, i: usize, m: &HashMap<(char, usize), char>) -> char {
        *m.get(&(c, i)).unwrap_or(&c)
    }

    fn shift_line(line: &str, i: usize, m: &HashMap<(char, usize), char>) -> String {
        line.chars().map(|c| shift_char(c, i, m)).collect()
    }

    fn rotations(line: &str, m: &HashMap<(char, usize), char>, odysseus: &Vec<&str>) -> usize {
        for i in 0..26 {
            for &o in odysseus {
                if shift_line(line, i, m).contains(o) {
                    return i;
                }
            }
        }
        0
    }

    let mut ans = 0;
    for line in lines {
        ans += rotations(line, &m, &odysseus);
    }

    println!("{}", ans);
}
