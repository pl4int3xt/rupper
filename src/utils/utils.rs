use text_to_ascii_art::convert;
use std::fs::File;
use std::io::{BufRead, BufReader};
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

pub fn check_verbose(verbose: &str, attempts: &i32, password: &Vec<u8>, password_hash: &str){
    match verbose {
        "true" => println!("[{}] [{}] == [{}]", attempts, std::str::from_utf8(&password).unwrap(), password_hash),
        "false" => {},
        _ => println!("Invalid verbose mode")
    }
}

pub fn crack_password(password_file: &String, hash:&String, verbose: &str, hash_type: &str){
    let mut attempts: i32 = 1;
    let password_list = File::open(password_file).unwrap();
    let reader: BufReader<File> = BufReader::new(password_list);
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