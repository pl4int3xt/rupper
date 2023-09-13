use text_to_ascii_art::convert;

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