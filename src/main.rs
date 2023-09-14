use std::fs::File;
use std::io::BufReader;
use clap::Parser;

mod utils;
mod algorithm;
mod handlers;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args{
    /// Name of algorithm [sha1, sha256, md2, md4, md5]
    #[arg(short, long)]
    algorithm: String,

    /// The hash value
    #[arg(short, long)]
    input: String,

    /// password list 
    #[arg(short, long)]
    password_list: String,

    /// verbose
    #[arg(short, long, default_value_t = 1)]
    verbose : u8,
}

fn main() {
    utils::utils::banner("Rupper");
    let args = Args::parse();

    let hash_type: String = args.algorithm;
    let hash: String = args.input;
    let password_file: String = args.password_list;
    let verbose: u8 = args.verbose;
    let password_list = File::open(password_file).unwrap();
    let reader: BufReader<File> = BufReader::new(password_list);

    utils::utils::crack_password(&hash,&verbose,&hash_type,reader)

}
