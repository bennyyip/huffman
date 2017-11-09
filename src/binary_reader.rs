use std::io::{Read, Bytes};
pub struct BinaryReader<T: Read> {
    buffer: u8,
    index: u8,
    inner: Bytes<T>,
}

impl<T: Read> BinaryReader<T> {
    pub fn new(r: T) -> Self {
        BinaryReader {
            buffer: 0,
            index: 8,
            reader: r.bytes(),
        }
    }

    pub fn read_bit(&mut self) -> u8 {
        self.index -= 1;
        let bit = (self.buffer >> self.index) & 1;
        if self.index == 0 {
            self.index = 8;
            if let Some(byte) = self.reader.next() {
                self.buffer = byte.unwrap();
            }
        }
        bit
    }

    pub fn read_u8(&mut self) -> u8 {
        let x = self.buffer << (8 - self.index);
        if let Some(byte) = self.reader.next() {
            self.buffer = byte.unwrap();
        }
        x | (self.buffer >> self.index)
    }

    pub fn read_u64(&mut self) -> u64 {
        let mut x = 0;
        for i in 0..4 {
            x |= self.read_u8() as u64;
            x <<= 8;
        }
        x
    }
}
