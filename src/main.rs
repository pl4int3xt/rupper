use std::fs::File;
use std::io::BufReader;
use clap::Parser;
mod utils;
mod algorithm;
mod handlers;


fn main() {
    utils::utils::banner("Rupper");
    let args = utils::utils::Args::parse();

    let hash_type: String = args.algorithm;
    let hash: String = args.input;
    let password_file: String = args.password_list;
    let verbose: u8 = args.verbose;
    let password_list = File::open(password_file).unwrap();
    let reader: BufReader<File> = BufReader::new(password_list);
    
    utils::utils::crack_password(&hash,&verbose,&hash_type,reader)

}
