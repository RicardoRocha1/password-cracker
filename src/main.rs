use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::env;
use std::process::exit;
use sha2::{Sha256, Digest};

fn main() {
    let hash_args: Vec<String> = env::args().collect();
    if hash_args.len() != 2 {
        eprintln!("\nInvalid format. \nValid format: 'cargo run <sha256 hash>'\n");
        exit(1);
    }

    let wanted_hash: &String = &hash_args[1];
    let password_file: &str = "Data/rockyou.txt";
    let mut attempts: u64 = 1;

    println!("Cracking: {}", wanted_hash);

    let password_list: Result<File, Error> = File::open(password_file);
    let reader: BufReader<File> = match password_list {
        Ok(file) => BufReader::new(file),
        Err(_) => {
            eprintln!("Error opening password file.");
            exit(1);
        }
    };

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => {
                eprintln!("Error reading line.");
                continue;
            }
        };

        if line.chars().any(|c| !c.is_ascii()) {
            println!("Skipping line with invalid characters.");
            continue;
        }

        let password: &str = &line.trim();
        let password_hash: String = format!("{:x}", Sha256::digest(password.as_bytes()));

        println!("[{}] {} == {}", attempts, password, password_hash);
        if password_hash == *wanted_hash {
            println!("Password found after {} attempts.\n{} hashes to {}.",attempts, password, password_hash);
            exit(0);
        }
        attempts += 1;
    }
    println!("Not found.\nTry using another password list");
}
