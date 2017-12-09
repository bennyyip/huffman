use std::io::{Read, Bytes};
pub struct BinaryReader<T: Read> {
    buffer: u8,
    index: i8,
    reader: Bytes<T>,
}

impl<T: Read> BinaryReader<T> {
    pub fn new(r: T) -> Self {
        BinaryReader {
            buffer: 0,
            index: -1,
            reader: r.bytes(),
        }
    }

    pub fn read_bit(&mut self) -> u8 {
        if self.index < 0 {
            if let Some(byte) = self.reader.next() {
                self.buffer = byte.unwrap();
                self.index = 7;
            } else {
                panic!("run out of input");
            }
        }
        let bit = (self.buffer >> self.index) & 1;
        self.index -= 1;
        bit
    }

    pub fn read_u8(&mut self) -> u8 {
        if self.index == -1 {
            self.buffer = 0;
        }
        let x = self.buffer.wrapping_shl((7 - self.index) as u32);

        if let Some(byte) = self.reader.next() {
            self.buffer = byte.unwrap();
        }

        x | (self.buffer >> (self.index + 1))
    }

    // big-endian
    pub fn read_u64(&mut self) -> u64 {
        let mut x = 0;
        for _ in 0..8 {
            x <<= 8;
            x |= u64::from(self.read_u8());
        }
        x
    }
}

#[test]
fn test_binary_read() {
    let data = [0, 0, 0, 0, 0, 0, 0, 0xb, 0x23, 0xFA, 0xCE];
    let mut r = BinaryReader::new(&data[..]);
    assert_eq!(r.read_u64(), 0xb);
    assert_eq!(r.read_u8(), 0x23);

    assert_eq!(r.read_bit(), 1);
    assert_eq!(r.read_bit(), 1);
    assert_eq!(r.read_bit(), 1);
    assert_eq!(r.read_bit(), 1);
    assert_eq!(r.read_bit(), 1);
    assert_eq!(r.read_bit(), 0);
    assert_eq!(r.read_bit(), 1);

    assert_eq!(r.read_u8(), 0b01100111);
    assert_eq!(r.read_bit(), 0);
}
