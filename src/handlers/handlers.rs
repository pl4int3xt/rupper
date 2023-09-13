use sha2::{Sha256, Digest};
use sha1::Sha1;
use md2::Md2;
use md4::Md4;
use md5;

use crate::algorithm::algorithm::HashAlgorithm;

pub fn assign_algorithm_type(hash_type: &str) -> HashAlgorithm {
    match hash_type {
        "md2" => HashAlgorithm::Md2, 
        "sha256" => HashAlgorithm::Sha256, 
        "md4" => HashAlgorithm::Md4,
        "sha1" => HashAlgorithm::Sha1,
        "md5" => HashAlgorithm::Md5,
        _ => HashAlgorithm::Invalid
    }
}

pub fn digest(hash_algorithm: &HashAlgorithm, password: &Vec<u8>) -> String { 
    let mut md2 = Md2::new();
    let mut md4 = Md4::new();
    match hash_algorithm {
        HashAlgorithm::Md2 => {
            md2.update(password);
            format!("{:x}", md2.finalize())
        },
        HashAlgorithm::Sha256 => format!("{:x}", Sha256::digest(password)),
        HashAlgorithm::Md4 => {
            md4.update(password);
            format!("{:x}", md4.finalize())
        },
        HashAlgorithm::Sha1 => format!("{:x}", Sha1::digest(password)),
        HashAlgorithm::Invalid => "Invalid algorithm".to_string(),
        HashAlgorithm::Md5 => format!("{:x}", md5::compute(password))          
    }
}