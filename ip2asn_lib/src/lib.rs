pub mod bgptools;

pub use self::bgptools::BgpTools;
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
    map: BTreeMap<(T, u8), u32>,
}

impl<T: std::cmp::Ord> AsnMap<T> {
    pub fn lookup(&self, key: impl Into<T>) -> Option<&u32> {
        // this is the bulk of how ip2asn works.
        // this finds the next smallest key to our query key and does so in
        // log2 time
        self.map
            .range(..=(key.into(), 128))
            .next_back()
            .map(|(_, v)| v)
    }
}

pub trait AsnMapper {
    fn parse(path: &Path) -> Result<(AsnMap<u32>, AsnMap<u128>), Error>;
}
