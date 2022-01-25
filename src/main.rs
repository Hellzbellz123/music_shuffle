use std::{env, fs};
use std::fs::{File};
use std::io::{BufRead, BufReader, self, Write};
use rand::seq::SliceRandom;
use rand::thread_rng;



fn main() {
    let input = env::args().nth(1);
    let reader: Box<dyn BufRead> = match input {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) => Box::new(BufReader::new(fs::File::open(filename).unwrap()))

    };
    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap());
    }
    lines.shuffle(&mut thread_rng());

    let mut outputfile = File::create("output.txt").expect("Unable to write file!");

    for line in lines{
        writeln!(outputfile, "{}", line).expect("There was an Error");
    }
}