pub mod binary_reader;
pub mod binary_writer;
pub mod huffman;
pub mod compress;
pub mod decompress;

pub use compress::{compress_file, show_freq_and_dict};
pub use decompress::decompress_file;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
