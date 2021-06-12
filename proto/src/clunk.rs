use crate::fcall;
use crate::header;
use crate::utils;
use std::fmt;

#[derive(Default)]
pub struct TClunk {
    pub header: header::Header,
    pub fid: u32,
}

impl fmt::Display for TClunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "tclunk: [{}, fid: {}]", self.header, self.fid,)
    }
}

impl fcall::Fcall for TClunk {
    fn set_header(&mut self, header: header::Header) {
        self.header = header;
    }
    fn get_tag(&self) -> u16 {
        self.header.get_tag()
    }

    fn compose(&self) -> Option<Vec<u8>> {
        let length = 4 + 1 + 2 + 4;
        let mut buffer: Vec<u8> = Vec::with_capacity(length);

        // let buf: &mut &[u8] = &mut &buffer[..];

        buffer.extend(&u32::to_le_bytes(length as u32));
        buffer.push(self.header.htype.unwrap() as u8);
        buffer.extend(&u16::to_le_bytes(self.header.htag));

        buffer.extend(self.fid.to_le_bytes().iter());

        Some(buffer)
    }

    fn parse(&mut self, buf: &mut &[u8]) {
        self.header.parse(buf);
        self.fid = utils::read_le_u32(buf);
    }
}

#[derive(Default)]
pub struct RClunk {
    pub header: header::Header,
}

impl fmt::Display for RClunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rclunk: [{}]", self.header,)
    }
}

impl fcall::Fcall for RClunk {
    fn set_header(&mut self, header: header::Header) {
        self.header = header;
    }
    fn get_tag(&self) -> u16 {
        self.header.get_tag()
    }

    fn compose(&self) -> Option<Vec<u8>> {
        let length = 4 + 1 + 2;
        let mut buffer: Vec<u8> = Vec::with_capacity(length);

        // let buf: &mut &[u8] = &mut &buffer[..];

        buffer.extend(&u32::to_le_bytes(length as u32));
        buffer.push(self.header.htype.unwrap() as u8);
        buffer.extend(&u16::to_le_bytes(self.header.htag));

        Some(buffer)
    }

    fn parse(&mut self, buf: &mut &[u8]) {
        self.header.parse(buf);
    }
}
