use crate::fcall;
use crate::header;
use crate::qid;
use crate::utils;
use std::fmt;

#[derive(Default)]
pub struct TWrite {
    pub header: header::Header,
    pub fid: u32,
    pub offset: u64,
    pub count: u32,
    pub data: Vec<u8>,
}

impl fmt::Display for TWrite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "twrite: [{}, fid: {}, offset: {}, count: {}]",
            self.header, self.fid, self.offset, self.count,
        )
    }
}

impl fcall::Fcall for TWrite {
    fn set_header(&mut self, header: header::Header) {
        self.header = header;
    }
    fn get_tag(&self) -> u16 {
        self.header.get_tag()
    }

    fn compose(&self) -> Option<Vec<u8>> {
        let length: usize = 4 + 1 + 2 + 4 + 8 + 4 + self.count as usize;
        let mut buffer: Vec<u8> = Vec::with_capacity(length);

        // let buf: &mut &[u8] = &mut &buffer[..];

        buffer.extend(&u32::to_le_bytes(length as u32));
        buffer.push(self.header.htype.unwrap() as u8);
        buffer.extend(&u16::to_le_bytes(self.header.htag));

        buffer.extend(&self.fid.to_le_bytes());
        buffer.extend(&self.offset.to_le_bytes());
        buffer.extend(&self.count.to_le_bytes());
        buffer.extend(&self.data);

        Some(buffer)
    }

    fn parse(&mut self, buf: &mut &[u8]) {
        self.header.parse(buf);
        self.fid = utils::read_le_u32(buf);
        self.offset = utils::read_le_u64(buf);
        self.count = utils::read_le_u32(buf);
        self.data = utils::read_string(buf).unwrap().as_bytes().to_vec();
    }
}

#[derive(Default)]
pub struct RWrite {
    pub header: header::Header,
    pub count: u32,
}

impl fmt::Display for RWrite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rwrite: [{}, count: {}]", self.header, self.count)
    }
}

impl fcall::Fcall for RWrite {
    fn set_header(&mut self, header: header::Header) {
        self.header = header;
    }
    fn get_tag(&self) -> u16 {
        self.header.get_tag()
    }

    fn compose(&self) -> Option<Vec<u8>> {
        let length: usize = 4 + 1 + 2 + 4;
        let mut buffer: Vec<u8> = Vec::with_capacity(length);

        buffer.extend(&u32::to_le_bytes(length as u32));
        buffer.push(self.header.htype.unwrap() as u8);
        buffer.extend(&u16::to_le_bytes(self.header.htag));

        buffer.extend(&self.count.to_le_bytes());

        Some(buffer)
    }

    fn parse(&mut self, buf: &mut &[u8]) {
        self.header.parse(buf);
        self.count = utils::read_le_u32(buf);
    }
}
