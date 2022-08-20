use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub fn get_lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {

  let file = File::open(filename).expect("File not found");
  let reader = BufReader::new(file);

  reader
    .lines()
    .collect::<Result<_, _>>()
    .unwrap()
}
