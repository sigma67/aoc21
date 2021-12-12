use std::{env, fs};

pub fn import_test_input(day: u8) -> String {
    read_test_input(format!("day{:02}_test.txt", day))
}

pub fn import_test_input_2(day: u8) -> String {
    read_test_input(format!("day{:02}_test2.txt", day))
}

fn read_test_input(filename: String) -> String {
    let cwd = env::current_dir().unwrap();
    let filename = cwd.join("inputs").join(filename);
    println!("Reading {}", filename.display());
    fs::read_to_string(filename).expect("Error while reading")
}