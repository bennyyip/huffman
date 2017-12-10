use std::io::Write;

pub struct BinaryWriter<T: Write> {
    buffer: u8,
    index: i8,
    inner: T,
}

impl<T: Write> BinaryWriter<T> {
    pub fn new(w: T) -> BinaryWriter<T> {
        BinaryWriter {
            buffer: 0,
            index: 8,
            inner: w,
        }
    }

    pub fn write_bit(&mut self, x: u8) {
        self.index -= 1;
        self.buffer |= x << self.index;
        if self.index == 0 {
            self.inner.write_all(&[self.buffer]).unwrap();
            self.index = 8;
            self.buffer = 0;
        }
    }

    pub fn write_u8(&mut self, x: u8) {
        self.inner
            .write_all(&[self.buffer | (x >> (8 - self.index))])
            .unwrap();
        if self.index == 8 {
            self.buffer = 0;
        } else {
            self.buffer = x << self.index as u32;
        }
    }

    // big-endian
    pub fn write_u64(&mut self, x: u64) {
        for i in (0..8).rev() {
            self.write_u8((x >> (i * 8)) as u8);
        }
    }
}

impl<T: Write> Drop for BinaryWriter<T> {
    fn drop(&mut self) {
        if self.index < 8 {
            self.inner.write_all(&[self.buffer]).unwrap();
        }
    }
}
