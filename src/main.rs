extern crate huffman;


use std::env;
use std::fs::File;
use huffman::{compress_file, decompress_file};

fn main() {

    let mut args = env::args();
    let _ = args.next().unwrap();
    let action = args.next().unwrap();
    let input_file = args.next().unwrap();
    let output_file = args.next().unwrap();

    let mut output = File::create(output_file).unwrap();

    if action == "-c" {
        compress_file(&input_file, &mut output).unwrap();
    } else {
        decompress_file(&input_file, &mut output).unwrap();
    }

}
