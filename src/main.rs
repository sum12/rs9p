use client;
use proto;

use proto::HeaderType;
use proto::Message;
use std::default::Default;
use std::io::Read;
use std::net::TcpStream;

fn main() {
    let s = TcpStream::connect("localhost:9999").unwrap();
    let stx = s.try_clone().unwrap();
    let mut client = client::Client::new(stx);

    let mut version: proto::TRVersion = Default::default();
    version.header = proto::Header {
        htype: Some(HeaderType::Tversion),
        htag: 0,
    };

    version.msize = 65536;
    version.version = "9P2000".to_string();
    let rversion = client.get_response(version);
    match rversion {
        Some(Message::RVersion(x)) => {
            client.msize = x.msize;
        }
        _ => panic!("version mismatch"),
    }

    let attach = proto::TAttach {
        header: proto::Header {
            htype: Some(proto::HeaderType::Tattach),
            htag: 0,
        },
        fid: 0,
        afid: !0u32,
        uname: "kyle".to_string(),
        aname: "".to_string(),
    };

    match client.get_response(attach) {
        Some(Message::RAttach(_x)) => {}
        _ => panic!("cloud not attach"),
    }

    let fpath = "/static".to_string();
    let mut r = match client.open(fpath.clone(), proto::Mode::Oread) {
        Some(r) => r,
        _ => panic!("Nope; cant open file"),
    };
    let mut buffer = String::new();
    match r.read_to_string(&mut buffer) {
        Ok(c) => {
            println!("{}", buffer)
        }
        _ => panic!("Nope; cant read"),
    }

    println!("done !");
}
