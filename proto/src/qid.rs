use crate::fcall;
use crate::header;
use crate::utils;
use std::fmt;

#[derive(Default)]
pub struct Qid {
    pub qtype: u8,
    pub vers: u32,
    pub uid: u64,
}

impl fmt::Display for Qid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "qtype: 0x{:X}, version: {}, uid: {}",
            self.qtype, self.vers, self.uid
        )
    }
}

impl fcall::Fcall for Qid {
    type Header = header::Header;
    fn set_header(&mut self, _header: Self::Header) {}
    fn get_tag(&self) -> u16 {
        unimplemented!()
    }
    fn compose(&self) -> Option<Vec<u8>> {
        let mut buffer: Vec<u8> = Vec::with_capacity(13 as usize);
        buffer.push(self.qtype);
        buffer.extend(&self.vers.to_le_bytes());
        buffer.extend(&self.uid.to_le_bytes());
        Some(buffer)
    }
    fn parse(&mut self, buf: &mut &[u8]) {
        self.qtype = utils::read_le_u8(buf);
        self.vers = utils::read_le_u32(buf);
        self.uid = utils::read_le_u64(buf);
    }
}
