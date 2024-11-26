use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn load_input_lines(f: &str) -> Vec<String> {
    match File::open(f) {
        Ok(v) => {
            let buffered = BufReader::new(v);
            let mut vec = Vec::<String>::new();
            for line in buffered.lines() {
                vec.push(line.unwrap());
            }
            vec
        },
        Err(e) => {
            panic!("{}", e);
        }
    }
}

pub fn load_input_line(f: &str) -> String {
    match File::open(f) {
        Ok(v) => {
            let buffered = BufReader::new(v);
            buffered.lines().next().unwrap().expect("Line not found in file")
        },
        Err(e) => {
            panic!("{}", e);
        }
    }
}