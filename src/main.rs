use std::error::Error;
use std::io::{self, Read, Write, ErrorKind, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::path::PathBuf;

fn read_dict_words<R>(reader:R) -> Result<Vec<String>, io::Error> where R: BufRead {
    let mut res: Vec<String> = Vec::new();

    for item in reader.lines().into_iter() {
        match item {
            Err(why) => Err(why),
            Ok(word) => { res.append(word) },
        }
    }

    Ok(res)
}

fn main() -> Result<(), Box<dyn Error>>{

    let mut args = std::env::args().skip(1);

    let dict_file_name = match args.next() {
        Some(s) => s,
        None => Err("usage: wordle FILE...")?
    };

    let mut file = match File::open(dict_file_name) {
        Err(why) => panic!("Couldn't open: {}", why),
        Ok(file) => file
    };

    let mut words: Vec<io::Result<String>> = BufReader::new(file).lines().collect();

    for word in words.into_iter() {
        match word {
            Err(why) => panic!("Not a word: {}", why),
            Ok(w) => println!("{}", w)
        }
    }

    Ok(())
}
