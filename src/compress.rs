use huffman::*;
use binary_writer::*;
use std::io::{Read, Write, Bytes, BufReader};
use std::fs::File;


pub fn read_freq_table<R: Read>(input: &mut Bytes<R>) -> [u64; 256] {
    let mut freq_table = [0; 256];
    while let Some(Ok(x)) = input.next() {
        freq_table[x as usize] += 1;
    }
    for (symbol, &freq) in freq_table.iter().enumerate() {
        if freq > 0 {
        }
    }
    freq_table
}

fn dfs<W: Write>(
    node: &Node,
    bit_sequence: &mut Vec<bool>,
    table: &mut [Vec<bool>],
    output: &mut BinaryWriter<W>,
) {
    match *node {
        Node::Leaf(x) => {
            output.write_bit(1);
            output.write_u8(x);
            table[x as usize] = bit_sequence.clone();
        }

        Node::Parent {
            ref left,
            ref right,
        } => {
            output.write_bit(0);

            bit_sequence.push(false);
            dfs(left, bit_sequence, table, output);

            let index = bit_sequence.len() - 1;
            bit_sequence[index] = true;
            dfs(right, bit_sequence, table, output);

            bit_sequence.pop();
        }
    }
}

pub fn compress<R: Read, W: Write>(
    freq_table: [u64; 256],
    input: &mut R,
    output: &mut BinaryWriter<W>,
) {
    let mut num_bytes = 0;

    // -1           if no symbol
    // -2           if more than one symbol
    // non-negative if only one symbol, = the symbol
    let mut single_symbol = -1;

    for (symbol, freq) in freq_table.iter().enumerate() {
        if *freq > 0 {
            num_bytes += *freq;
            if single_symbol >= 0 {
                single_symbol = -2;
            } else if single_symbol == -1 {
                single_symbol = symbol as i32;
            }
        }
    }

    output.write_u64(num_bytes);

    if num_bytes == 0 {
        return;
    }

    // whole file consists of the same byte
    if single_symbol >= 0 {
        output.write_bit(1);
        output.write_u8(single_symbol as u8);
        return;
    }

    if let Some(root) = build_tree(freq_table) {
        let mut table = vec![vec![]; 256];

        let mut bit_sequence = vec![];
        dfs(&root, &mut bit_sequence, &mut table, output);


        let mut input_bytes = input.bytes();
        while let Some(Ok(x)) = input_bytes.next() {
            for &bit in &table[x as usize] {
                output.write_bit(if bit { 1 } else { 0 });
            }
        }
    }
}

pub fn compress_file<W: Write>(input_file: &str, output: &mut W) -> ::std::io::Result<()> {
    let freq_table = {
        let f = File::open(input_file)?;
        let mut input = BufReader::new(f).bytes();
        read_freq_table(&mut input)
    };

    let f = File::open(input_file)?;
    let mut input = BufReader::new(f);
    let mut output = BinaryWriter::new(output);
    compress(freq_table, &mut input, &mut output);
    Ok(())
}
