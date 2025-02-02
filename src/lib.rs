use std::{
    fs,
    io,
};

const SILCROW : &str = "§";
const WHIM : &str = "whim/";

pub fn is_section(line: &str) -> bool {
    if line.starts_with(SILCROW) {
        true
    } else {false}
}

pub fn whimsward(file_name: &str) -> Result {
    let full_path = WHIM + file_name;
    let result = fs::read_to_string(full_path);
}

