use client;
use proto;

use proto::header::HeaderType;
use std::default::Default;
use std::net::TcpStream;

fn main() {
    let s = TcpStream::connect("localhost:9999").unwrap();
    let stx = s.try_clone().unwrap();
    let mut client = client::Client::new(stx);

    let mut version: Box<proto::version::TRVersion> = Default::default();
    version.header = proto::header::Header {
        htype: Some(HeaderType::Tversion),
        htag: 0,
    };

    version.msize = 65536;
    version.version = "9P2000".to_string();
    let rversion = client.get_response(version);
    println!("{}", rversion.unwrap());

    let attach = Box::new(proto::attach::TAttach {
        header: proto::header::Header {
            htype: Some(proto::header::HeaderType::Tattach),
            htag: 0,
        },
        fid: 0,
        afid: !0u32,
        uname: "kyle".to_string(),
        aname: "".to_string(),
    });

    let rattach = client.get_response(attach);
    println!("{}", rattach.unwrap());

    println!("done !");
}
