#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

//pub mod attach;
mod attach;
pub use attach::*;
mod clunk;
pub use clunk::*;
mod error;
pub use error::*;
mod fcall;
pub use fcall::*;
mod header;
pub use header::*;
mod qid;
pub use qid::*;
mod utils;
pub use utils::*;
mod version;
pub use version::*;
mod walk;
pub use walk::*;
mod open;
pub use open::*;

pub type Fcallbox = Box<dyn fcall::Fcall>;
pub enum Message {
    TAttach(TAttach),
    RAttach(RAttach),
    TClunk(TClunk),
    RClunk(RClunk),
    TWalk(TWalk),
    RWalk(RWalk),
    TVersion(TRVersion),
    RVersion(TRVersion),
    RError(RError),
    Header(Header),
    Qid(Qid),
}

macro_rules! implement {
    ($to:ident, $from:ty) => {
        impl From<$from> for Message {
            fn from(val: $from) -> Message {
                Message::$to(val)
            }
        }

        impl From<Vec<u8>> for $from {
            fn from(val: Vec<u8>) -> $from {
                let mut ret: Self = Default::default();
                let b = &mut &val[..];
                ret.parse(b);
                ret
            }
        }
    };
}

impl From<TRVersion> for Message {
    fn from(val: TRVersion) -> Message {
        match val.header.htype {
            Some(HeaderType::Tversion) => Message::TVersion(val),
            Some(HeaderType::Rversion) => Message::RVersion(val),
            _ => unreachable!(),
        }
    }
}

impl From<Vec<u8>> for TRVersion {
    fn from(val: Vec<u8>) -> TRVersion {
        let mut ret: Self = Default::default();
        let b = &mut &val[..];
        ret.parse(b);
        ret
    }
}

// implement!(TVersion, TRVersion);
// implement!(RVersion, TRVersion);

implement!(TAttach, TAttach);
implement!(RAttach, RAttach);
implement!(TWalk, TWalk);
implement!(RWalk, RWalk);
implement!(TClunk, TClunk);
implement!(RClunk, RClunk);
implement!(RError, RError);
implement!(Header, Header);
implement!(Qid, Qid);
impl Message {
    pub fn new(h: header::HeaderType, buf: Vec<u8>) -> Message {
        match h {
            HeaderType::Tversion | HeaderType::Rversion => TRVersion::from(buf).into(),
            HeaderType::Tattach => TAttach::from(buf).into(),
            HeaderType::Rattach => RAttach::from(buf).into(),
            HeaderType::Twalk => TWalk::from(buf).into(),
            HeaderType::Rwalk => RWalk::from(buf).into(),
            HeaderType::Tclunk => TClunk::from(buf).into(),
            HeaderType::Rclunk => RClunk::from(buf).into(),
            HeaderType::Rerror => RError::from(buf).into(),
            _ => todo!(),
        }
    }
    //     pub fn new(hdr: header::Header) -> Self {
    //         let mut f: &dyn fcall::Fcall = match hdr.htype.unwrap() {
    //             header::HeaderType::Tversion => &version::TRVersion::default(),
    //             // header::HeaderType::Tattach => attach::TAttach::default().into(),
    //             // header::HeaderType::Rattach => attach::RAttach::default().into(),
    //             // header::HeaderType::Rerror => error::RError::default().into(),
    //             _ => {
    //                 println!("invalid header type after reading bytes");
    //                 panic!("ERR")
    //             }
    //         };
    //         f.set_header(hdr);
    //         f.into()
    //     }
}
//
// impl From<Box<dyn fcall::Fcall>> for Message {
//     fn from(x: Box<dyn fcall::Fcall>) -> Self {
//         x.into()
//

// impl Deref for Message {
//     type Target = Fcallbox;
//     fn deref(&self) -> &Self::Target {
//         match self {
//             Message::TAttach(x) => x,
//             // Message::RAttach(x) => x,
//             // Message::TClunk(x) => x,
//             // Message::RClunk(x) => x,
//             // Message::TWalk(x) => x,
//             // Message::RWalk(x) => x,
//             // Message::TVersion(x) => x,
//             // Message::RVersion(x) => x,
//             // Message::RError(x) => x,
//             _ => unreachable!(),
//         }
//     }
// }
//
// impl DerefMut for Message {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         match self {
//             Message::TAttach(x) => x,
//             // Message::RAttach(x) => x,
//             // Message::TClunk(x) => x,
//             // Message::RClunk(x) => x,
//             // Message::TWalk(x) => x,
//             // Message::RWalk(x) => x,
//             // Message::TVersion(x) => x,
//             // Message::RVersion(x) => x,
//             // Message::RError(x) => x,
//             _ => unreachable!(),
//         }
//     }
// }
//
// impl<T: Fcall> From<T> for Message {
//     fn from(x: T) -> Self {
//         Message::My(x)
//     }
// }
//

// impl Message {
//     pub fn get_tag(&self) -> u16 {
//         match self {
//             Message::TVersion(x) => x.get_tag(),
//             Message::RVersion(x) => x.get_tag(),
//         }
//     }
// }

// impl Deref for Message {
//     type Target = dyn Fcall;
//     fn deref(&self) -> &Self::Target {
//         match self {
//             Message::TVersion(x) | Message::RVersion(x) => x,
//         }
//     }
// }

// impl From<Box<dyn Fcall>> for Message {
//     fn from(val: Box<dyn Fcall>) -> Message {
//         match val.get_header_type() {
//             Some(HeaderType::Tversion) => Message::TVersion(val),
//             Some(HeaderType::Rversion) => Message::RVersion(val),
//             _ => unreachable!(),
//         }
//     }
// }

// impl<T: Fcall + Default> From<&mut &[u8]> for T {
//     fn from(s: &mut &[u8]) -> T {
//         let mut ret: T = <T as Default>::default();
//         ret.parse(s);
//         ret
//     }
// }
