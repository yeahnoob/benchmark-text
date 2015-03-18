#![feature(unicode)]
#![feature(str_words)]

//! Store a file into a HashMap of Vector.
//! Benchmark the language implemetation's I/O, Hash and Map performance.

extern crate unicode;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;
use std::env;
//use std::str::StrExt;
use unicode::str::UnicodeStr;

/// store word pairs into the HashMap of Vector `textpairs`.
fn main() {
    let arguments: Vec<String> = env::args().map(|x| x.to_string()).collect();
    println!("reading the file: \"{}\"", arguments[1]);
    let file = match File::open(&arguments[1]) {
        Ok(file) => file,
        Err(..) => panic!("Cannot find the File with the given filename"),
    };
    let reader= BufReader::new(&file);
    let alllines: Vec<String> = reader.lines().map(|x| x.ok().unwrap()).collect();

    let mut textpairs: HashMap< &str, Vec<&str> > = HashMap::new();
    for line in alllines.iter() {
        let twowords: Vec<&str> = line.words().collect();
        let fw = twowords[0];
        let lw = twowords[1];

        /*
        if twowords.len() < 2 {
            //if debug {
            // println!("the last line two words: {}...{}", last_firstword, last_lastword);
            println!("the panic word: {:?}", twowords);
            panic!("The input file ERROR.\n One panic line at least, which only contains One Word!!!");
        }
        */

        if !textpairs.contains_key(&fw) {
            textpairs.insert(fw, Vec::new());
        } else {
            textpairs.get_mut(&fw).unwrap().push(lw);
        }
    }

    println!("{}", textpairs.get(&*arguments[2]).unwrap().len());
}
