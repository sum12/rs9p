use crate::fcall;
use crate::utils;
use std::convert::TryFrom;
use std::fmt;

pub struct Header {
    pub htype: Option<HeaderType>,
    pub htag: u16,
}

impl std::default::Default for Header {
    fn default() -> Self {
        Header {
            htype: None,
            htag: 0,
        }
    }
}
impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "tag: {}, type: {:?}", self.htag, self.htype)
    }
}

impl fcall::Fcall for Header {
    fn set_header(&mut self, _header: Header) {}

    fn get_tag(&self) -> u16 {
        self.htag
    }

    fn parse(&mut self, buf: &mut &[u8]) {
        if buf.len() < 3 {
            panic!("header too small");
        } else {
            let htype = utils::read_le_u8(buf);
            self.htype = Some(HeaderType::try_from(htype).unwrap());
            self.htag = utils::read_le_u16(buf);
        }
    }

    fn compose(&self) -> Option<Vec<u8>> {
        Some(vec![0u8])
    }
}

#[derive(Clone, Copy, Debug)]
pub enum HeaderType {
    Tversion = 100,
    Rversion,
    Tauth,
    Rauth,
    Tattach,
    Rattach,
    Terror,
    Rerror,
    Tflush,
    Rflush,
    Twalk,
    Rwalk,
    Topen,
    Ropen,
    Tcreate,
    Rcreate,
    Tread,
    Rread,
    Twrite,
    Rwrite,
    Tclunk,
    Rclunk,
    Tremove,
    Rremove,
    Tstat,
    Rstat,
    Twstat,
    Rwstat,
}

impl TryFrom<u8> for HeaderType {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            100 => Ok(Self::Tversion),
            101 => Ok(Self::Rversion),
            102 => Ok(Self::Tauth),
            103 => Ok(Self::Rauth),
            104 => Ok(Self::Tattach),
            105 => Ok(Self::Rattach),
            106 => Ok(Self::Terror),
            107 => Ok(Self::Rerror),
            108 => Ok(Self::Tflush),
            109 => Ok(Self::Rflush),
            110 => Ok(Self::Twalk),
            111 => Ok(Self::Rwalk),
            112 => Ok(Self::Topen),
            113 => Ok(Self::Ropen),
            114 => Ok(Self::Tcreate),
            115 => Ok(Self::Rcreate),
            116 => Ok(Self::Tread),
            117 => Ok(Self::Rread),
            118 => Ok(Self::Twrite),
            119 => Ok(Self::Rwrite),
            120 => Ok(Self::Tclunk),
            121 => Ok(Self::Rclunk),
            122 => Ok(Self::Tremove),
            123 => Ok(Self::Rremove),
            124 => Ok(Self::Tstat),
            125 => Ok(Self::Rstat),
            126 => Ok(Self::Twstat),
            127 => Ok(Self::Rwstat),
            _ => Err(format!("Invalie headertype: {}", value)),
        }
    }
}
