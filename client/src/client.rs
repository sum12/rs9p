use crate::file;
use proto;
use proto::Fcall;
use proto::Message;
use proto::{Header, HeaderType};
use std::collections::HashMap;
use std::default::Default;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct Client {
    pub resp_map: Arc<Mutex<HashMap<u16, Sender<Message>>>>,
    pub rootfid: u32,
    pub srx: TcpStream,
    pub closed: Arc<Mutex<bool>>,
    pub tags: Mutex<Vec<u16>>,
    pub last_tag: u16,
    pub fids: Mutex<Vec<u32>>,
    pub last_fid: u32,
    pub msize: u32,
}

impl Client {
    pub fn new(io: TcpStream) -> Self {
        let client = Client {
            resp_map: Default::default(),
            rootfid: Default::default(),
            srx: io,
            closed: Default::default(),
            tags: Default::default(),
            last_tag: 1,
            fids: Default::default(),
            last_fid: Default::default(),
            msize: Default::default(),
        };

        let resp_table = client.resp_map.clone();
        let mut srx = client.srx.try_clone().unwrap();
        let closed = client.closed.clone();

        thread::spawn(move || loop {
            if *closed.lock().unwrap() {
                println!("thread closed");
                return;
            }
            match parse_call(&mut srx) {
                Some((h, f)) => {
                    let _ = match resp_table.lock().unwrap().remove(&h.get_tag()) {
                        Some(tx) => Ok(tx.send(f.into()).unwrap()),
                        _ => {
                            println!("tx not found");
                            Err("tag not presend in hashmap")
                        }
                    };
                    ()
                }
                _ => {
                    println!("breaking from read loop");
                    break;
                }
            }
        });
        client
    }
}
pub fn parse_call(input: &mut TcpStream) -> Option<(Header, Message)> {
    let mut sizebuff = [0u8; 4];

    match input.read(&mut sizebuff) {
        Ok(4) => {
            let length = (proto::read_le_u32(&mut &sizebuff[..]) - 4) as usize;
            let mut buf = vec![0u8; length];
            match input.read_exact(&mut buf) {
                Ok(()) => {
                    let mut h = Header::default();
                    h.parse(&mut &buf[..]);
                    match h.htype {
                        Some(htype) => Some((h, Message::new(htype, buf))),
                        _ => {
                            println!("invalid header type after reading {} bytes", length);
                            panic!("ERR")
                        }
                    }
                }
                _ => {
                    println!("could not get exact");
                    None
                }
            }
        }
        _ => {
            println!("could not get 4");
            None
        }
    }
}

fn take<T: std::ops::AddAssign + Copy>(from: &Mutex<Vec<T>>, x: &mut T, y: T) -> T {
    from.lock().unwrap().pop().unwrap_or_else(|| {
        *x += y;
        *x
    })
}

impl Client {
    pub fn get_response<U: Fcall + Into<Message>>(&mut self, fcall: U) -> Option<Message> {
        let rx = {
            let (tx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();
            let tag = fcall.get_tag();
            let _ = self.resp_map.lock().unwrap().insert(tag, tx);
            rx
        };
        match self.srx.write_all(&fcall.compose().unwrap()) {
            Ok(_) => println!("{}", fcall),
            _ => panic!("error writing {}", fcall),
        };
        let fcall = rx.recv().unwrap();
        println!("{}", fcall.extract());
        Some(fcall.into())
    }

    pub fn take_fid(&mut self) -> u32 {
        take(&self.fids, &mut self.last_fid, 1)
    }

    pub fn take_tag(&mut self) -> u16 {
        take(&self.tags, &mut self.last_tag, 1)
    }

    pub fn return_tag(&self, tag: u16) {
        if tag != 0 {
            self.tags.lock().unwrap().push(tag);
            self.resp_map.lock().unwrap().remove(&tag);
        }
    }

    pub fn return_fid(&self, fid: u32) {
        self.fids.lock().unwrap().push(fid);
    }

    pub fn clunk(&mut self, fid: u32) {
        if fid != 0 {
            let mut clunk: proto::TClunk = Default::default();
            let mut h: Header = Default::default();
            h.htag = self.take_tag();
            clunk.set_header(h);
            clunk.fid = fid;
            self.get_response(clunk);
            self.return_fid(fid);
        }
    }

    pub fn walkfid(&mut self, path: String) -> Option<u32> {
        let parts: Vec<String> = path
            .split("/")
            .map(|s| s.to_string())
            .filter(|s| s.len() > 0)
            .collect();
        // println!("{:?}", parts);
        let newfid = self.take_fid();
        let walk = proto::TWalk {
            header: Header {
                htype: Some(HeaderType::Twalk),
                htag: self.take_tag(),
            },
            fid: self.rootfid,
            newfid: newfid,
            nwname: parts.len() as u16,
            wname: parts,
        };

        let resp = self.get_response(walk);
        match resp {
            Some(Message::RWalk { .. }) => Some(newfid),
            Some(Message::RError(x)) => {
                println!("RError");
                println!("{}", x);
                self.clunk(newfid);
                None
            }
            _ => {
                self.clunk(newfid);
                None
            }
        }
    }

    pub fn open(&mut self, path: String, mode: proto::Mode) -> Option<file::File> {
        let newfid = match self.walkfid(path) {
            Some(x) => x,
            _ => return None,
        };
        let open = proto::TOpen {
            header: Header {
                htype: Some(HeaderType::Topen),
                htag: self.take_tag(),
            },
            fid: newfid,
            mode,
        };

        match self.get_response(open) {
            Some(Message::ROpen(x)) => Some(file::File {
                client: self,
                fid: newfid,
                offset: 0,
                iounit: if x.iounit == 0 { u32::MAX } else { x.iounit },
            }),

            Some(Message::RError(x)) => {
                println!("RError");
                println!("{}", x);
                self.clunk(newfid);
                None
            }
            _ => None,
        }
    }
}
