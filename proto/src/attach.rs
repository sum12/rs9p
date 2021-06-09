use crate::fcall;
use crate::header;
use crate::qid;
use crate::utils;
use std::convert::TryInto;
use std::fmt;

#[derive(Default)]
pub struct TAttach {
    pub header: header::Header,
    pub fid: u32,
    pub afid: u32,
    pub uname: String,
    pub aname: String,
}

impl fmt::Display for TAttach {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "tattach: [{}, fid: {}, afid: {}, uname: {}, aname:{}]",
            self.header, self.fid, self.afid, self.uname, self.aname
        )
    }
}

impl fcall::Fcall for TAttach {
    type Header = header::Header;
    fn set_header(&mut self, header: Self::Header) {
        self.header = header;
    }
    fn get_tag(&self) -> u16 {
        self.header.get_tag()
    }
    fn parse(&mut self, buf: &mut &[u8]) {
        self.fid = utils::read_le_u32(buf);
        self.afid = utils::read_le_u32(buf);
        self.uname = utils::read_string(buf).unwrap();
        self.aname = utils::read_string(buf).unwrap();
    }

    fn compose(&self) -> Option<Vec<u8>> {
        let length: u32 =
            4 + 1 + 2 + 4 + 4 + (2 + self.uname.len() as u32) + (2 + self.aname.len() as u32);
        let mut buffer: Vec<u8> = Vec::with_capacity(length as usize);
        buffer.extend(&length.to_le_bytes());
        buffer.push(self.header.htype.unwrap() as u8);
        buffer.extend(&self.header.htag.to_le_bytes());
        buffer.extend(&self.fid.to_le_bytes());
        buffer.extend(&self.afid.to_le_bytes());
        buffer.extend(&u16::to_le_bytes(self.uname.len().try_into().unwrap()));
        buffer.extend(self.uname.as_bytes());
        buffer.extend(&u16::to_le_bytes(self.aname.len().try_into().unwrap()));
        buffer.extend(self.aname.as_bytes());
        Some(buffer)
    }
}

#[derive(Default)]
pub struct RAttach {
    pub header: header::Header,
    pub qid: qid::Qid,
}

impl fmt::Display for RAttach {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rattach: [{}, qid: {}]", self.header, self.qid)
    }
}
impl fcall::Fcall for RAttach {
    type Header = header::Header;
    fn set_header(&mut self, header: Self::Header) {
        self.header = header;
    }
    fn get_tag(&self) -> u16 {
        self.header.get_tag()
    }
    fn parse(&mut self, buf: &mut &[u8]) {
        self.qid.parse(buf);
    }

    fn compose(&self) -> Option<Vec<u8>> {
        todo!()
    }
}
