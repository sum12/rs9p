use crate::fcall::Fcall;
use crate::header;
use crate::utils;
// use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt;

#[derive(Default)]
pub struct TRVersion {
    pub header: header::Header,
    pub msize: u32,
    pub version: String,
}

impl fmt::Display for TRVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self.header.htype {
            Some(header::HeaderType::Tversion) => 't',
            _ => 'r',
        };
        write!(
            f,
            "{}version:[{}, msize: {}, version: {}]",
            c, self.header, self.msize, self.version
        )
    }
}

impl Fcall for TRVersion {
    fn set_header(&mut self, header: header::Header) {
        self.header = header;
    }
    fn get_tag(&self) -> u16 {
        self.header.get_tag()
    }
    fn compose(&self) -> Option<Vec<u8>> {
        let length = 4 + 1 + 2 + 4 + (2 + self.version.len());
        let mut buffer: Vec<u8> = Vec::with_capacity(length);

        // let buf: &mut &[u8] = &mut &buffer[..];

        buffer.extend(&u32::to_le_bytes(length as u32));
        buffer.push(self.header.htype.unwrap() as u8);
        buffer.extend(&self.header.htag.to_le_bytes());
        buffer.extend(&self.msize.to_le_bytes());
        buffer.extend(&u16::to_le_bytes(self.version.len().try_into().unwrap()));
        buffer.extend(self.version.as_bytes());
        Some(buffer)
    }
    fn parse(&mut self, buf: &mut &[u8]) {
        self.header.parse(buf);
        self.msize = utils::read_le_u32(buf);
        self.version = utils::read_string(buf).unwrap();
    }
}

// impl From<&mut &[u8]> for TRVersion {
//     fn from(s: &mut &[u8]) -> TRVersion {
//         let mut ret: Self = Default::default();
//         ret.parse(s);
//         ret
//     }
// }
//
// impl TryFrom<&mut [u8]> for TRVersion {
//     type Error = String;
//     fn try_from(value: &mut [u8]) -> Result<Self, Self::Error> {
//         self.msize = utils::read_le_u32(buf) as usize;
//         self.version = utils::read_string(buf).unwrap();
//         Some(buf)
//     }
// }
//
//
