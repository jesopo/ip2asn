pub mod bgptools;

pub use self::bgptools::BgpTools;
use cidr::{Ipv4Cidr, Ipv6Cidr};
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Debug)]
pub enum Error {
    File(std::io::Error),
    Json(serde_json::Error),
}

impl From<std::io::Error> for Error {
    fn from(other: std::io::Error) -> Self {
        Error::File(other)
    }
}

impl From<serde_json::Error> for Error {
    fn from(other: serde_json::Error) -> Self {
        Error::Json(other)
    }
}

pub struct AsnMap<T: std::cmp::Ord> {
    map: BTreeMap<T, u32>,
}

impl<T: std::cmp::Ord> AsnMap<T> {
    pub fn lookup(&self, key: &T) -> Option<&u32> {
        self.map.range(..=key).next_back().map(|(_, v)| v)
    }
}

pub trait AsnMapper {
    fn parse(path: &Path) -> Result<(AsnMap<Ipv4Cidr>, AsnMap<Ipv6Cidr>), Error>;
}
