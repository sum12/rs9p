use crate::client;
use proto;
use std::convert::TryInto;
use std::io;
use std::rc;

pub struct File<'file> {
    pub client: &'file mut client::Client,
    pub fid: u32,
    pub offset: u64,
    pub iounit: u32,
}

impl io::Write for File<'_> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        todo!()
    }
    fn flush(&mut self) -> io::Result<()> {
        todo!()
    }
}

impl io::Read for File<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        //TODO: THIS IS AWKWARD
        let p = &mut buf[..];
        let x = &mut [0u8][..];
        let (p, x) = if p.len() > (self.client.msize - 11).try_into().unwrap() {
            p.split_at_mut((self.client.msize - 11).try_into().unwrap())
        } else {
            (p, x)
        };
        let (p, _) = if p.len() > self.iounit.try_into().unwrap() {
            p.split_at_mut(self.iounit.try_into().unwrap())
        } else {
            (p, x)
        };

        let read = proto::TRead {
            header: proto::Header {
                htype: Some(proto::HeaderType::Tread),
                htag: self.client.take_tag(),
            },
            count: p.len().try_into().unwrap(),
            fid: self.fid,
            offset: self.offset,
        };
        match self.client.get_response(read) {
            Some(proto::Message::RError(x)) => {
                println!("read failed: {}", x);
                Err(io::Error::new(std::io::ErrorKind::Other, x.ename))
            }
            Some(proto::Message::RRead(x)) => {
                self.offset += x.count as u64;
                if x.count > 0 {
                    p[..x.count as usize].copy_from_slice(&x.data);
                }
                Ok(x.count as usize)
            }
            _ => panic!("error reading file {}", self.fid),
        }
    }
}

impl File<'_> {
    fn twrite(&mut self, buf: Vec<u8>, offset: u64) -> Result<u32, String> {
        let mut wrote: u32 = 0;
        let mut min: u32;

        let p: &mut &[u8] = &mut &buf[..];

        loop {
            if wrote == p.len().try_into().unwrap() {
                break;
            }
            min = (self.client.msize - 23).try_into().unwrap();
            if p.len() > min as usize {
                *p = &mut &p[..min as usize]
            }
            let write = proto::TWrite {
                header: proto::Header {
                    htype: Some(proto::HeaderType::Twrite),
                    htag: self.client.take_tag(),
                },
                fid: self.fid,
                offset: self.offset,
                count: p.len().try_into().unwrap(),
                data: p.to_vec(),
            };
            match self.client.get_response(write) {
                Some(proto::Message::RError(x)) => {
                    println!("Write failed");
                    break;
                }
                Some(proto::Message::RWrite(x)) => {
                    wrote += x.count;
                    let (_, p) = p.split_at(x.count as usize);
                }
                _ => panic!("error writing file {}", self.fid),
            }
        }
        Ok(wrote)
    }
}
