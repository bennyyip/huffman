extern crate huffman;

use std::env;
use std::fs::File;
use std::io::BufWriter;
use huffman::{compress_file, decompress_file, show_freq_and_dict};

fn main() {

    let mut args = env::args();

    // generate freq_table and the dict for homework
    if args.len() == 2 {
        let _ = args.next().unwrap();
        let input_file = args.next().unwrap();
        show_freq_and_dict(&input_file).unwrap();
    }

    // compress
    if args.len() == 4 {
        let _ = args.next().unwrap();
        let action = args.next().unwrap();
        let input_file = args.next().unwrap();
        let output_file = args.next().unwrap();

        let mut output = BufWriter::new(File::create(output_file).unwrap());

        if action == "-c" {
            compress_file(&input_file, &mut output).unwrap();
        } else {
            decompress_file(&input_file, &mut output).unwrap();
        }
    }
}
