use std::env;
use std::fs::File;
use std::io::BufReader;

mod utils;
mod algorithm;
mod handlers;

fn main() {
    let args: Vec<String> = env::args().collect();
    utils::utils::banner("Rupper");
    utils::utils::invalid_arguments(&args);

    let hash_type: &String = &args[1];
    let hash: &String = &args[2];
    let password_file: &String = &args[3];
    let verbose: &String = &args[4];

    println!("Attempting to crack {}", hash);
    let password_list = File::open(password_file).unwrap();
    let reader: BufReader<File> = BufReader::new(password_list);

    utils::utils::crack_password(
        &hash,
        &verbose,
        &hash_type,
        reader
    )

}
