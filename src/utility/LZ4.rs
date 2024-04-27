use lz4_flex::block::{compress_prepend_size, decompress_size_prepended, DecompressError};

use crate::structure::{
    enums::StoneTransferProtocol,
    structs::define::{StructStone, StructStonePayload},
    traits::define::Detector,
};

pub trait CompressHandler {
    fn lz4_compress(&mut self);
    fn lz4_decompress(&mut self) -> Result<(), DecompressError>;
}

impl CompressHandler for Vec<u8> {
    fn lz4_compress(&mut self) {
        let compressed = compress_prepend_size(self.as_slice());
        self.clear();
        self.extend(compressed);
    }

    fn lz4_decompress(&mut self) -> Result<(), DecompressError> {
        match decompress_size_prepended(self.as_slice()) {
            Ok(decompress) => {
                self.clear();
                self.extend(decompress);
                return Ok(());
            }
            Err(err) => Err(err)
        }
    }
}

impl CompressHandler for StructStone {
    fn lz4_compress(&mut self) {
        self.set(
            StructStonePayload::build(true,
                                      StoneTransferProtocol::type_check(
                                          self.take_header().take_stone_type()
                                      ),
                                      self.take_payload().get_non_empty_data(),
            ).packet()
        );
    }

    fn lz4_decompress(&mut self) -> Result<(), DecompressError> {
        let mut payload = self.take_payload().get_non_empty_data();
        payload.lz4_decompress()?;
        self.set(
            StructStonePayload::build(
                false,
                StoneTransferProtocol::type_check(
                    self.take_header().take_stone_type()
                ),
                payload,
            ).packet()
        );
        Ok(())
    }
}