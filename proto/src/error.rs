use crate::fcall;
use crate::header;
use crate::utils;
use std::convert::TryInto;
use std::fmt;

#[derive(Default)]
pub struct RError {
    pub header: header::Header,
    pub ename: String,
}

impl fmt::Display for RError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rerror: [{}, ename: {}]", self.header, self.ename)
    }
}

impl fcall::Fcall for RError {
    fn set_header(&mut self, header: header::Header) {
        self.header = header;
    }
    fn get_tag(&self) -> u16 {
        self.header.get_tag()
    }
    fn compose(&self) -> Option<Vec<u8>> {
        let length = 4 + 1 + 2 + 4 + (2 + self.ename.len());
        let mut buffer: Vec<u8> = Vec::with_capacity(length);

        buffer.extend(&u32::to_le_bytes(length as u32));
        buffer.push(self.header.htype.unwrap() as u8);
        buffer.extend(&u16::to_le_bytes(self.header.htag));

        buffer.extend(&u16::to_le_bytes(self.ename.len().try_into().unwrap()));
        buffer.extend(self.ename.as_bytes());
        Some(buffer)
    }
    fn parse(&mut self, buf: &mut &[u8]) {
        self.header.parse(buf);
        self.ename = utils::read_string(buf).unwrap();
    }
}
