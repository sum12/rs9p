use crate::fcall;
use crate::header;
use crate::qid;
use crate::utils;
use std::convert::TryInto;
use std::fmt;

#[derive(Default)]
pub struct TWalk {
    pub header: header::Header,
    pub fid: u32,
    pub newfid: u32,
    pub nwname: u16,
    pub wname: Vec<String>,
}

impl fmt::Display for TWalk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "twalk: [{}, fid: {}, newfid: {}, nwname: {}, wname: <{}{}>]",
            self.header,
            self.fid,
            self.newfid,
            self.nwname,
            self.wname[0],
            self.wname[1..].join(",")
        )
    }
}

impl fcall::Fcall for TWalk {
    fn set_header(&mut self, header: header::Header) {
        self.header = header;
    }
    fn get_tag(&self) -> u16 {
        self.header.get_tag()
    }

    fn compose(&self) -> Option<Vec<u8>> {
        let mut length = 4 + 1 + 2 + 4 + 4 + 2;
        self.wname.iter().for_each(|s| length += 2 + s.len());

        let mut buffer: Vec<u8> = Vec::with_capacity(length);

        // let buf: &mut &[u8] = &mut &buffer[..];

        buffer.extend(&u32::to_le_bytes(length as u32));
        buffer.push(self.header.htype.unwrap() as u8);
        buffer.extend(&u16::to_le_bytes(self.header.htag));

        buffer.extend(&self.fid.to_le_bytes());
        buffer.extend(&self.newfid.to_le_bytes());
        buffer.extend(&self.nwname.to_le_bytes());

        self.wname.iter().for_each(|name| {
            buffer.extend(&u16::to_le_bytes(name.len().try_into().unwrap()));
            buffer.extend(name.as_bytes());
        });

        Some(buffer)
    }

    fn parse(&mut self, buf: &mut &[u8]) {
        self.header.parse(buf);
        self.fid = utils::read_le_u32(buf);
        self.newfid = utils::read_le_u32(buf);
        self.nwname = utils::read_le_u16(buf);
        self.wname = Vec::with_capacity(self.nwname.into());
        for _ in 0..self.nwname {
            self.wname.push(utils::read_string(buf).unwrap());
        }
    }
}

#[derive(Default)]
pub struct RWalk {
    pub header: header::Header,
    pub nwqid: u16,
    pub wqid: Vec<qid::Qid>,
}

impl fmt::Display for RWalk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "rwalk: [{}, nwqid: {}, nwqid: <{}>]",
            self.header,
            self.nwqid,
            self.wqid
                .iter()
                .map(|q| format!("{}", q))
                .collect::<String>()
        )
    }
}

impl fcall::Fcall for RWalk {
    fn set_header(&mut self, header: header::Header) {
        self.header = header;
    }
    fn get_tag(&self) -> u16 {
        self.header.get_tag()
    }

    fn compose(&self) -> Option<Vec<u8>> {
        let length = 4 + 1 + 2 + 2 + (self.nwqid * 13);
        let mut buffer: Vec<u8> = Vec::with_capacity(length.into());

        // let buf: &mut &[u8] = &mut &buffer[..];

        buffer.extend(&u32::to_le_bytes(length as u32));
        buffer.push(self.header.htype.unwrap() as u8);
        buffer.extend(&u16::to_le_bytes(self.header.htag));

        buffer.extend(self.nwqid.to_le_bytes().iter());

        let _ = self
            .wqid
            .iter()
            .map(|q| buffer.extend(q.compose().unwrap()));

        Some(buffer)
    }

    fn parse(&mut self, buf: &mut &[u8]) {
        self.header.parse(buf);
        self.nwqid = utils::read_le_u16(buf);
        self.wqid = Vec::with_capacity(self.nwqid.into());
        for _ in 0..self.nwqid {
            let mut q: qid::Qid = Default::default();
            q.parse(buf);
            self.wqid.push(q);
        }
    }
}
