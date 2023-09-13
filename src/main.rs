use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

mod algorithm;
mod handlers;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("Invalid amount of arguments");
        println!("Example: cargo run <algorithm> <sha256 hash> <pass list>");
        println!("Algorithms: <sha1> <sha256> <md2> <md4> <md5>");
        exit(1);
    }

    let hash_type: &String = &args[1];
    let hash: &String = &args[2];
    let password_file: &String = &args[3];
    let mut attempts: i32 = 1;

    println!("Attempting to crack {}", hash);
    let password_list = File::open(password_file).unwrap();
    let reader: BufReader<File> = BufReader::new(password_list);

    for line in reader.lines(){
        let line: String = line.unwrap();
        let password: Vec<u8> = line.trim().to_owned().into_bytes();
        let hash_algorithm: algorithm::algorithm::HashAlgorithm = handlers::handlers::assign_algorithm_type(hash_type);
        let password_hash: String = handlers::handlers::digest(&hash_algorithm, &password);
        
        if password_hash == "Invalid algorithm"{
            println!("Invalid algorithm");
            break;
        }

        println!("[{}] {} == {}", attempts, std::str::from_utf8(&password).unwrap(), password_hash);

        if &password_hash == hash {
            println!("Password found after {} attempts! {} hashes to {}!", attempts, std::str::from_utf8(&password).unwrap(), password_hash);
            break;
        }

        attempts +=1;
    }
    
    println!("Not found");
}
