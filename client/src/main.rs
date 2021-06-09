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

type Fcallbox = Box<dyn Fcall<Header = Header>>;

pub struct Client {
    resp_map: Arc<Mutex<HashMap<u16, Sender<Fcallbox>>>>,
    srx: TcpStream,
    tags: Mutex<Vec<u16>>,
    last_tag: u16,
    fids: Mutex<Vec<u32>>,
    last_fid: u32,
    msize: u32,
}

fn parse_call(input: &mut TcpStream) -> Option<Fcallbox> {
    let mut sizebuff = [0u8; 4];

    match input.read(&mut sizebuff) {
        Ok(4) => {
            let length = (proto::utils::read_le_u32(&mut &sizebuff[..]) - 4) as usize;
            let mut buf = vec![0; length];
            match input.read(&mut buf) {
                Ok(length) => {
                    let mut h: proto::header::Header = Default::default();
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

fn get_response(
    fcall: Fcallbox,
    resp_map: Arc<Mutex<HashMap<u16, Sender<Fcallbox>>>>,
    mut srx: TcpStream,
) -> Option<Fcallbox> {
    let (tx, rx): (Sender<Fcallbox>, Receiver<Fcallbox>) = mpsc::channel();
    let tag = fcall.get_tag();
    let _ = resp_map.lock().unwrap().insert(tag, tx);
    match srx.write_all(&fcall.compose().unwrap()) {
        Ok(_) => {
            println!("{}", fcall);
        }
        _ => panic!("error writing {}", fcall),
    };
    let fcall = rx.recv().ok();
    let _ = resp_map.lock().unwrap().remove(&tag);
    fcall
}

fn main() {
    let resp_map: Arc<Mutex<HashMap<u16, Sender<Fcallbox>>>> = Default::default();
    let s = TcpStream::connect("localhost:9999").unwrap();
    let mut stx = s.try_clone().unwrap();
    let resp_table = resp_map.clone();

    thread::spawn(move || loop {
        match parse_call(&mut stx) {
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

    let mut version: Box<proto::version::TRVersion> = Default::default();
    version.header = proto::header::Header {
        htype: Some(HeaderType::Tversion),
        htag: 0,
    };

    version.msize = 65536;
    version.version = "9P2000".to_string();
    let rversion = get_response(version, resp_map.clone(), s.try_clone().unwrap());
    println!("{}", rversion.unwrap());

    let attach = proto::attach::TAttach {
        header: proto::header::Header {
            htype: Some(proto::header::HeaderType::Tattach),
            htag: 0,
        },
        fid: 0,
        afid: !0u32,
        uname: "kyle".to_string(),
        aname: "".to_string(),
    };

    println!("done !");
}
