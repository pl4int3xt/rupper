use std::env;
use std::process::exit;

mod utils;
mod algorithm;
mod handlers;

fn main() {
    let args: Vec<String> = env::args().collect();
    utils::utils::banner("Rupper");

    if args.len() != 5 {
        println!("Invalid amount of arguments");
        println!("Example: cargo run <algorithm> <sha256 hash> <pass list> <verbose> <threads>");
        println!("Algorithms: <sha1> <sha256> <md2> <md4> <md5>");
        exit(1);
    }

    let hash_type: &String = &args[1];
    let hash: &String = &args[2];
    let password_file: &String = &args[3];
    let verbose: &String = &args[4];
    // let threads: &i32 = &args[5].parse::<i32>().unwrap();

    // let mut handles: Vec<i32> = vec![];

    println!("Attempting to crack {}", hash);

    utils::utils::crack_password(password_file, hash, verbose, hash_type)

}
