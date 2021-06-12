use crate::fcall;
use crate::header;
use crate::qid;
use crate::utils;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::default;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum Mode {
    Oread = 0,
    Owrite,
    Ordwr,
    Oexec,
    Onone,
    Otrunc = 0x10,
    Orclose = 0x40,
}

pub const IOUNIT: u32 = 16384;
impl default::Default for Mode {
    fn default() -> Self {
        Mode::Oread
    }
}

impl TryFrom<u8> for Mode {
    type Error = String;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Mode::Oread),
            1 => Ok(Mode::Owrite),
            2 => Ok(Mode::Ordwr),
            3 => Ok(Mode::Oexec),
            4 => Ok(Mode::Onone),

            0x10 => Ok(Mode::Otrunc),
            0x40 => Ok(Mode::Orclose),
            _ => Err("Protocol Error: UnSupported mode value".to_string()),
        }
    }
}

#[derive(Default)]
pub struct TOpen {
    pub header: header::Header,
    pub fid: u32,
    pub mode: Mode,
}

impl fmt::Display for TOpen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "topen: [{}, fid: {}, mod: {:?}]",
            self.header, self.fid, self.mode,
        )
    }
}

impl fcall::Fcall for TOpen {
    fn set_header(&mut self, header: header::Header) {
        self.header = header;
    }
    fn get_tag(&self) -> u16 {
        self.header.get_tag()
    }

    fn compose(&self) -> Option<Vec<u8>> {
        let length = 4 + 1 + 2 + 4 + 1;
        let mut buffer: Vec<u8> = Vec::with_capacity(length);

        // let buf: &mut &[u8] = &mut &buffer[..];

        buffer.extend(&u32::to_le_bytes(length as u32));
        buffer.push(self.header.htype.unwrap() as u8);
        buffer.extend(&u16::to_le_bytes(self.header.htag));

        buffer.extend(&self.fid.to_le_bytes());
        buffer.extend(&u8::to_le_bytes(self.mode as u8));

        Some(buffer)
    }

    fn parse(&mut self, buf: &mut &[u8]) {
        self.header.parse(buf);
        self.fid = utils::read_le_u32(buf);
        match utils::read_le_u8(buf).try_into() {
            Ok(x) => self.mode = x,
            _ => unreachable!(),
        }
    }
}

#[derive(Default)]
pub struct ROpen {
    pub header: header::Header,
    pub qid: qid::Qid,
    pub iounit: u32,
}

impl fmt::Display for ROpen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "wopen: [{}, qid: {}, iounit: {}]",
            self.header, self.qid, self.qid,
        )
    }
}

impl fcall::Fcall for ROpen {
    fn set_header(&mut self, header: header::Header) {
        self.header = header;
    }
    fn get_tag(&self) -> u16 {
        self.header.get_tag()
    }

    fn compose(&self) -> Option<Vec<u8>> {
        let length = 4 + 1 + 2 + 13 + 4;
        let mut buffer: Vec<u8> = Vec::with_capacity(length);

        // let buf: &mut &[u8] = &mut &buffer[..];

        buffer.extend(&u32::to_le_bytes(length as u32));
        buffer.push(self.header.htype.unwrap() as u8);
        buffer.extend(&u16::to_le_bytes(self.header.htag));

        let qid_buf = self.qid.compose().unwrap();

        buffer.extend(qid_buf);
        buffer.extend(&self.iounit.to_le_bytes());

        Some(buffer)
    }

    fn parse(&mut self, buf: &mut &[u8]) {
        self.header.parse(buf);
        self.qid.parse(buf);
        self.iounit = utils::read_le_u32(buf);
    }
}
