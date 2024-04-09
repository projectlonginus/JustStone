use std::io::{Read, Write};
use lz4::{Decoder, EncoderBuilder};

pub fn LZ4_compress(source: &mut Vec<u8>) -> (&mut Vec<u8>, std::io::Result<()>) {
    let mut encoder = EncoderBuilder::new()
        .level(4)
        .build(source).unwrap();
    encoder.finish()
}


// pub fn LZ4_decompress(source: &mut Vec<u8>, packet: &mut [u8]) -> (&[u8], std::io::Result<()>) {
//     let mut decoder = Decoder::new(&source[..])?;
//     decoder.read_exact(packet)?;
//     decoder.finish()
// }