use std::{env, fs};

pub fn import_test_input(day: u8) -> String {
    let cwd = env::current_dir().unwrap();
    let filename = cwd.join("inputs").join(format!("day{:02}_test.txt", day));
    println!("Reading {}", filename.display());
    fs::read_to_string(filename).expect("Error while reading")
}