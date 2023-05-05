use std::cmp::{Ord, Ordering};

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

#[derive(Clone, Debug)]
pub struct NetRange<T: Ord + Clone> {
    pub min: T,
    pub max: T,
    pub len: u8,
    pub asn: u32,
}

impl<T: Clone + Ord> std::cmp::PartialEq for NetRange<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len.eq(&other.len)
    }
}
impl<T: Clone + Ord> std::cmp::Eq for NetRange<T> {}

impl<T: Clone + Ord> std::cmp::PartialOrd for NetRange<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.len.cmp(&other.len))
    }
}

impl<T: Clone + Ord> std::cmp::Ord for NetRange<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.len.cmp(&other.len)
    }
}
