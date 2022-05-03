use rand::Rng;
use std::collections::HashMap;
use std::fs::File;
use std::{io::*, vec};

// fn give_rand(line_range: usize) -> usize {
//     rand::thread_rng().gen_range(0..line_range)
// }

fn dice_roll(dice_count: i32) -> Vec<i32> {
    let mut rand_rolls = vec![];
    for _ in 0..dice_count {
        let random_roll = rand::thread_rng().gen_range(1..7);
        rand_rolls.push(random_roll);
    }
    rand_rolls
}

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
    println!("");
    println!("");
    println!("-------------------------");
    println!("randomly generated words for password;");
    println!("\n");
    for _ in 0..5 {
        let rolls = dice_roll(5);

        let mut roll_string = String::new();
        for d_roll in rolls {
            let converted_roll = d_roll.to_string();
            roll_string.push_str(&converted_roll);
        }
        
        // println!("{}", roll_string);
        // println!("{:?}", rolls);

        let file_name = "eff_large_wordlist.txt";
        let details = get_file(file_name);

        // Get value from file.
        // let cat = details.get("11124");
        let secret_word = details.get(&roll_string);
        if let Some(value) = &secret_word {
            // println!("VALUE FOUND: {}", value);
            print!("{} ", value);
        }

    }
    println!("");
    println!("");
    println!("-------------------------");
    println!("");
}
