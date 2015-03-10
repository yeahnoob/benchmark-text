#![feature(old_path)]
#![feature(old_io)]
#![feature(str_words)]

//! Store a file into a HashMap of Vector.
//! Benchmark the language implemetation's I/O, Hash and Map performance.

use std::old_io::BufferedReader;
use std::old_io::File;
use std::collections::HashMap;
use std::env;

/// store word pairs into the HashMap of Vector `textpairs`.
fn main() {
    let arguments: Vec<String> = env::args().map(|x| x.to_string()).collect();
    println!("reading the file: \"{}\"", arguments[1]);
    let path = Path::new(&*arguments[1]);
    let mut file = BufferedReader::new(File::open(&path));
    let alllines: Vec<String> = file.lines().map(|x| x.unwrap()).collect();

    // if debug {
    // let mut last_firstword: String = "".to_string();
    // let mut last_lastword: String = "".to_string();
    // }

    let mut textpairs: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in alllines.iter() {
        let twowords: Vec<&str> = line.words().collect();

        /*
        if twowords.len() < 2 {
            //if debug {
            // println!("the last line two words: {}...{}", last_firstword, last_lastword);
            println!("the panic word: {:?}", twowords);
            panic!("The input file ERROR.\n One panic line at least, which only contains One Word!!!");
        }
        */

        let firstword = twowords[0];
        let lastword = twowords[1];

        // if debug {
        // last_firstword = firstword.to_string();
        // last_lastword = lastword.to_string();
        // }

        if !textpairs.contains_key(&firstword) {
            textpairs.insert(&firstword, Vec::new());
        } else {
            textpairs.get_mut(&firstword).unwrap().push(&lastword);
        }
    }

    println!("{:?}", textpairs.get(&*arguments[2]).unwrap().len());
}
