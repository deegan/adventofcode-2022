use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let lines = BufReader::new(File::open("lines.txt").unwrap()).lines().collect::<Vec<_>>();
    let elfs = lines.split(" ");
    for elf in &elfs {
        println!("{}", elf)
        }
    }
