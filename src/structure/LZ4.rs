use std::io::Read;

use lz4::{Decoder, EncoderBuilder};

pub fn lz4_compress(source: &mut Vec<u8>) -> (&mut Vec<u8>, std::io::Result<()>) {
    let mut encoder = EncoderBuilder::new()
        .level(4)
        .build(source).unwrap();
    encoder.finish()
}

pub fn lz4_decompress(source: &mut Vec<u8>) -> Vec<u8> {
    let mut decoder = Decoder::new(&source[..]).unwrap();
    let mut packet = Vec::with_capacity(source.capacity());
    decoder.read_exact(&mut packet[..]).unwrap();
    let (o, _r) = decoder.finish();
    println!("{:?}", _r);
    o.to_vec()
}