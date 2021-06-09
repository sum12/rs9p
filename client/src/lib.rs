use proto;
use proto::fcall::Fcall;
use proto::header::{Header, HeaderType};
use std::collections::HashMap;
use std::default::Default;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub type Fcallbox = Box<dyn Fcall<Header = Header>>;

pub struct Client {
    resp_map: Arc<Mutex<HashMap<u16, Sender<Fcallbox>>>>,
    rootFid: u32,
    srx: TcpStream,
    closed: Arc<Mutex<bool>>,
    tags: Mutex<Vec<u16>>,
    last_tag: u16,
    fids: Mutex<Vec<u32>>,
    last_fid: u32,
    msize: u32,
}

impl Client {
    pub fn new(io: TcpStream) -> Self {
        let client = Client {
            resp_map: Default::default(),
            rootFid: Default::default(),
            srx: io,
            closed: Default::default(),
            tags: Default::default(),
            last_tag: Default::default(),
            fids: Default::default(),
            last_fid: Default::default(),
            msize: Default::default(),
        };

        let resp_table = client.resp_map.clone();
        let srx = client.srx.try_clone().unwrap();
        let closed = client.closed.clone();

        thread::spawn(move || loop {
            if *closed.lock().unwrap() {
                return;
            }
            match parse_call(&mut srx.try_clone().unwrap()) {
                Some(f) => {
                    let _ = match resp_table.lock().unwrap().get(&f.get_tag()) {
                        Some(tx) => Ok(tx.send(f).unwrap()),
                        _ => Err("tag not presend in hashmap"),
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
pub fn parse_call(input: &mut TcpStream) -> Option<Fcallbox> {
    let mut sizebuff = [0u8; 4];

    match input.read(&mut sizebuff) {
        Ok(4) => {
            let length = (proto::utils::read_le_u32(&mut &sizebuff[..]) - 4) as usize;
            let mut buf = vec![0; length];
            match input.read(&mut buf) {
                Ok(length) => {
                    let mut h: Header = Default::default();
                    let b = &mut &buf[..];
                    h.parse(b);
                    let mut f: Fcallbox = match h.htype {
                        Some(HeaderType::Tversion) | Some(HeaderType::Rversion) => {
                            Box::new(proto::version::TRVersion::default())
                        }
                        Some(HeaderType::Tattach) => Box::new(proto::attach::TAttach::default()),
                        Some(HeaderType::Rattach) => Box::new(proto::attach::RAttach::default()),
                        _ => {
                            println!("invalid header type after reading {} bytes", length);
                            panic!("ERR")
                        }
                    };
                    f.set_header(h);
                    f.parse(b);
                    Some(f)
                }
                _ => None,
            }
        }
        _ => None,
    }
}

impl Client {
    pub fn get_response(&mut self, fcall: Fcallbox) -> Option<Fcallbox> {
        let (tx, rx): (Sender<Fcallbox>, Receiver<Fcallbox>) = mpsc::channel();
        let tag = fcall.get_tag();
        let _ = self.resp_map.lock().unwrap().insert(tag, tx);
        match self.srx.write_all(&fcall.compose().unwrap()) {
            Ok(_) => {
                println!("{}", fcall);
            }
            _ => panic!("error writing {}", fcall),
        };
        let fcall = rx.recv().ok();
        let _ = self.resp_map.lock().unwrap().remove(&tag);
        fcall
    }
}
