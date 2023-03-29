pub mod bgptools;

pub use self::bgptools::BgpTools;
use cidr::{Cidr, Ipv4Cidr, Ipv6Cidr};
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Debug)]
pub enum Error {
    /// Error whilst reading provided IP<->ASN mapping file
    File(std::io::Error),
    /// Error whilst JSON-decoding IP<->ASN mappings
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

pub struct AsnMap<T: Cidr> {
    map: BTreeMap<T, u32>,
}

impl<T: Cidr> AsnMap<T> {
    /// Find which CIDR is the closest superset of the provided CIDR.
    pub fn lookup(&self, key: &T) -> Option<&u32> {
        // this is the centerpiece of how ip2asn works.
        //
        // `self.map.range(..=key)` creates a potential range from the start
        // of the map, through everything less-than-or-equal-to the query key
        //
        // `.next_back()` then visits only the end of that potential range
        if let Some((cidr, asn)) = self.map.range(..=key).next_back() {
            // given we only know the network we've found is
            // less-than-or-equal-to the query key, we need to make sure we're
            // not past the end of the network
            (cidr.contains(&key.first_address())).then_some(asn)
        } else {
            None
        }
    }
}

pub trait AsnMapper {
    /// Take a path to an IP<->ASN mapping file, read it, and parse it.
    ///
    /// # Errors
    ///
    /// Will return `Err` if there was an issue reading or parsing IP<->ASN
    /// mapping file.
    fn parse(path: &Path) -> Result<(AsnMap<Ipv4Cidr>, AsnMap<Ipv6Cidr>), Error>;
}
