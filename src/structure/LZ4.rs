use lz4_flex::block::{compress_prepend_size, decompress_size_prepended, DecompressError};
use crate::structure::{Detector, PacketBuilder, StoneTransferProtocol, StructStone, StructStoneHeader, StructStonePayload, TypeManager};

pub trait CompressHandler {
    fn lz4_compress(&mut self);
    fn lz4_decompress(&mut self) -> Result<(), DecompressError>;
}

impl CompressHandler for Vec<u8> {
    fn lz4_compress(&mut self){
        let compressed = compress_prepend_size(self.as_slice());
        self.clear();
        self.extend(compressed);
    }

    fn lz4_decompress(&mut self) -> Result<(), DecompressError> {
        match decompress_size_prepended(self.as_slice()) {
            Ok(decompress) => { self.clear(); self.extend(decompress); return Ok(()) },
            Err(err) => Err(err)
        }
    }
}

impl CompressHandler for StructStone {
    fn lz4_compress(&mut self){
        self.get_header().set_stone_status(vec![1, 0, 0, 0]);
        self.set(Self::build(self.get_header(), self.get_payload()));
    }

    fn lz4_decompress(&mut self) -> Result<(), DecompressError> {
        let mut gs = self.get_sysinfo();
        let mut gc = self.get_command();
        let mut gr = self.get_response();
        let mut gf = self.get_file();
        gs.lz4_decompress()?;
        gc.lz4_decompress()?;
        gr.lz4_decompress()?;
        gf.lz4_decompress()?;
        self.set(PacketBuilder::from(false,
                 StoneTransferProtocol::type_check(self.take_header().take_stone_type()),
                 StructStonePayload::from(gs, gc, gr, gf)).packet());
        Ok(())
    }
}
