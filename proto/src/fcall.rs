use std::fmt;

pub trait Fcall: fmt::Display + Send {
    type Header;
    fn set_header(&mut self, header: Self::Header);
    fn get_tag(&self) -> u16;
    fn compose(&self) -> Option<Vec<u8>>;
    fn parse(&mut self, buf: &mut &[u8]);
}
