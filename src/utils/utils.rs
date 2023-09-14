use text_to_ascii_art::convert;
use std::io::{BufRead, BufReader};
use std::fs::File;
use crate::algorithm::algorithm::HashAlgorithm;
use crate::handlers::handlers::{assign_algorithm_type, digest};


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

pub fn check_verbose(verbose: &u8, attempts: &i32, password: &Vec<u8>, password_hash: &str){
    match verbose {
        1 => println!("[{}] [{}] == [{}]", attempts, std::str::from_utf8(&password).unwrap(), password_hash),
        0 => {},
        _ => println!("Invalid verbose mode")
    }
}

pub fn crack_password(hash:&String, verbose: &u8, hash_type: &str, reader: BufReader<File>){
    println!("Attempting to crack {}\n", hash);
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
            println!("Password found\n");
            println!("-----------------------------------------------------------------------------------\n");
            println!("Attempts [{}]\n", attempts);
            println!("Password [{}]\n", std::str::from_utf8(&password).unwrap());
            println!("Hash [{}]\n", password_hash);
            println!("-----------------------------------------------------------------------------------");
            break;
        }

        attempts +=1;
    }
}