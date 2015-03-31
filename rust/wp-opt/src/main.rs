#![feature(core)]
#![feature(unicode)]
#![feature(str_words)]

//! Store a file into a HashMap of Vector, with a different algorithm.
//! Benchmark the language implemetation's I/O, Hash and Map performance.

extern crate unicode;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;
use std::env;
use unicode::str::UnicodeStr;
use std::num::wrapping::Wrapping;

// my hash function, changing a string into a uint64 number.
fn myhash(s: &str) -> u64 {
    let mut ha = Wrapping(0x1505u64);
    let hachars: Vec<char> = s.chars().collect();
    for c in hachars {
        ha = (((ha << 5) + ha) >> 3) + Wrapping(c as u64);
    }
    ha.0
}

fn main() {
    let arguments: Vec<String> = env::args().map(|x| x.to_string()).collect();
    println!("reading the file: \"{}\"", arguments[1]);
    let file = match File::open(&arguments[1]) {
        Ok(file) => file,
        Err(..) => panic!("Cannot find the File with the given filename"),
    };
    let reader = BufReader::new(&file);
    let alllines: Vec<String>
        = reader.lines().map(|x| x.ok().unwrap()).collect();
    // hashedpairs := < hashed_first_word_of_pairs, Vec<the_line_number_with_the_same_hash> >
    let mut hashedpairs: HashMap< u64, Vec<u64> > = HashMap::new();
    let mut linenumber: u64 = 0;
    for line in alllines.iter() {
        let twowords: Vec<&str> = line.words().collect();
        let fwhash = myhash(twowords[0]);
        linenumber += 1;
        if !hashedpairs.contains_key(&fwhash) {
            hashedpairs.insert(fwhash, Vec::new());
        } else {
            hashedpairs.get_mut(&fwhash).unwrap().push(linenumber);
        }
    }
    
    println!("{}", hashedpairs.get(&myhash(&*arguments[2])).unwrap().len());
}
