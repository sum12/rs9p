use crate::header::Header;
use std::fmt;

pub trait Fcall: fmt::Display + Send {
    fn set_header(&mut self, header: Header);
    fn get_tag(&self) -> u16;
    fn compose(&self) -> Option<Vec<u8>>;
    fn parse(&mut self, buf: &mut &[u8]);
}
