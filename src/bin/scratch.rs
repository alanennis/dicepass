use std::collections::HashMap;
use std::fs::File;
use std::io::*;

fn get_file(filename: &str) -> HashMap<String, String> {
    // Open file of key-value pairs.
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut hash: HashMap<String, String> = HashMap::new();

    // Read and parse file.
    for line in reader.lines() {
        let line_inner = line.unwrap();
        let values: Vec<&str> = line_inner.split('\t').collect();
        if values.len() == 2 {
            hash.insert(values[0].to_string(), values[1].to_string());
        }
    }
    hash
}

fn main() {
    let file_name = "eff_large_wordlist.txt";
    let details = get_file(file_name);

    // Get value from file.
    let cat = details.get("11124");
    if let Some(value) = &cat {
        println!("VALUE FOUND: {}", value);
    }
}
