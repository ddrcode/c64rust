use serde_derive::Deserialize;
use std::fs::{read_to_string, File};
use std::io::Read;
use std::path::PathBuf;
use toml;

#[derive(Deserialize, Debug)]
pub struct Profile {
    rom: String,
}

pub fn get_file_as_byte_vec(filename: &PathBuf) -> Vec<u8> {
    let mut f = File::open(filename).expect("no file found");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("buffer overflow");
    buffer
}

pub fn load_profile_file(filename: &str) -> Profile {
    let f = PathBuf::from(filename);
    let contents = read_to_string(f).unwrap();
    let profile: Profile = toml::from_str(&contents).unwrap();
    println!("Profile {:?}", profile);
    profile
}
