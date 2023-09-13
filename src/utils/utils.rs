use text_to_ascii_art::convert;
use std::io::{BufRead, BufReader};
use std::process::exit;
use std::fs::File;
use crate::algorithm::algorithm::HashAlgorithm;
use crate::handlers::handlers::{assign_algorithm_type, digest};

pub fn invalid_arguments(args: &Vec<String>){
    if args.len() != 5 {
        println!("Invalid amount of arguments");
        println!("Example: cargo run <algorithm> <sha256 hash> <pass list> <verbose> <threads>");
        println!("Algorithms: <sha1> <sha256> <md2> <md4> <md5>");  
        exit(1);
    }
}
pub fn banner(banner: &str){
    match convert(banner.to_string()) {
        Ok(string) => println!("{}", string),
        Err(err) => println!("Error: {}", err)   
    }
    println!("");
    println!(" -------------------");
    println!("| Author: <Myst3ry> |");
    println!(" -------------------");
    println!("");
}

pub fn check_verbose(verbose: &str, attempts: &i32, password: &Vec<u8>, password_hash: &str){
    match verbose {
        "true" => println!("[{}] [{}] == [{}]", attempts, std::str::from_utf8(&password).unwrap(), password_hash),
        "false" => {},
        _ => println!("Invalid verbose mode")
    }
}

pub fn crack_password(hash:&String, verbose: &str, hash_type: &str, reader: BufReader<File>){
    let mut attempts: i32 = 1;
    for line in reader.lines(){
        let line: String = line.unwrap();
        let password: Vec<u8> = line.trim().to_owned().into_bytes();
        let hash_algorithm: HashAlgorithm = assign_algorithm_type(hash_type);
        let password_hash: String = digest(&hash_algorithm, &password);
        
        if password_hash == "Invalid algorithm"{
            println!("Invalid algorithm");
            break;
        }

        check_verbose(verbose, &attempts, &password, &password_hash); 
    
        if &password_hash == hash {
            println!("Password found");
            println!("-----------------------------------------------------------------------------------");
            println!("Attempts [{}]", attempts);
            println!("Password [{}]", std::str::from_utf8(&password).unwrap());
            println!("Hash [{}]", password_hash);
            println!("-----------------------------------------------------------------------------------");
            break;
        }

        attempts +=1;
    }
}