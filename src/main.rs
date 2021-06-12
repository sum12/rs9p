use client;
use proto;

use proto::HeaderType;
use proto::Message;
use std::default::Default;
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
            println!("{}", x);
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
        Some(Message::RAttach(x)) => println!("{}", x),
        _ => panic!("cloud not attach"),
    }

    let r = client.walkfid("/static".to_string());
    println!("{}", r.unwrap());

    println!("done !");
}
