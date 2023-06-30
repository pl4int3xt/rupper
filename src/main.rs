use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use sha2::{Sha256, Digest};
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args.collect();
    if args.len() != 3 {
        println!("Invalid amount of arguments");
        println!("Example: cargo run <sha256 hash>");
        exit(1);
    }

    let hash: &String = &args[1];
    let password: &String = &args[2];
    let mut attempts: i32 = 1;

    println!("Attempting to crack {}", hash);
}
