//! Store a file into a HashMap of Vector, with a different algorithm.
//! Benchmark the language implemetation's I/O, Hash and Map performance.

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;
use std::env;
use std::num::Wrapping;

// djb2 hash function(http://www.cse.yorku.ca/~oz/hash.html),
// changing a string into a uint64 number.
fn djb2hash(s: &str) -> u64 {
    let mut ha = Wrapping(5381u64);
    let hachars: Vec<char> = s.chars().collect();
    for c in hachars {
        ha = ((ha << 5) + ha) + Wrapping(c as u64);
    }
    ha.0
}

// Get filename(such as "word-pairs.txt") and keyword(such as "her")
// from command line arguments, read the file contents,
// hash the word pairs into a Dict( or Map named ),
// and count the specified keyword from the Dict finally.
fn main() {
    let arguments: Vec<String> = env::args().map(|x| x.to_string()).collect();
    println!("reading the file: \"{}\"", arguments[1]);
    let file = match File::open(&arguments[1]) {
        Ok(file) => file,
        Err(..) => panic!("Cannot find the File with the given filename"),
    };
    let reader = BufReader::new(&file);
    let alllines: Vec<String> = reader
        .lines()
        .map(|x| x.ok().unwrap())
        .collect();
    // hashedpairs := < hashed_first_word_of_pairs, Vec<the_line_number_with_the_same_hash> >
    let mut hashedpairs: HashMap< u64, Vec<u64> > = HashMap::new();
    let mut linenumber: u64 = 0;
    for line in alllines.iter() {
        let twowords: Vec<&str> = line
            .split(|c: char| c.is_whitespace())
            .filter(|s: &&str| !s.is_empty())
            .collect();
        let fwhash = djb2hash(twowords[0]);
        linenumber += 1;
        if !hashedpairs.contains_key(&fwhash) {
            hashedpairs.insert(fwhash, Vec::new());
        } else {
            hashedpairs.get_mut(&fwhash).unwrap().push(linenumber);
        }
    }
    
    println!("{}", hashedpairs.get(&djb2hash(&*arguments[2])).unwrap().len());
}
