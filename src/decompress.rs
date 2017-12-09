use huffman::*;
use binary_reader::*;
use std::io::{Read, Write, BufReader};
use std::fs::File;

fn dfs<R: Read>(input: &mut BinaryReader<R>) -> Node {
    if input.read_bit() == 1 {
        let symbol = input.read_u8();
        Node::Leaf(symbol)
    } else {
        let left = dfs(input);
        let right = dfs(input);
        Node::Parent {
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

pub fn decompress<R: Read, W: Write>(
    input: &mut BinaryReader<R>,
    output: &mut W,
) -> ::std::io::Result<()> {
    let mut bytes_left = input.read_u64();
    if bytes_left == 0 {
        return Ok(());
    }

    let root = dfs(input);
    if let Node::Leaf(x) = root {
        output.write_all(&vec![x; bytes_left as usize])?;
        return Ok(());
    }

    let mut node = &root;

    while bytes_left > 0 {
        match *node {
            Node::Leaf(x) => {
                output.write_all(&[x])?;
                bytes_left -= 1;
                node = &root;
            }
            Node::Parent {
                ref left,
                ref right,
            } => node = if input.read_bit() == 0 { left } else { right },
        }
    }
    Ok(())
}

pub fn decompress_file<W: Write>(input_file: &str, output: &mut W) -> ::std::io::Result<()> {
    let f = File::open(input_file)?;
    let mut input = BinaryReader::new(BufReader::new(f));
    decompress(&mut input, output)
}
