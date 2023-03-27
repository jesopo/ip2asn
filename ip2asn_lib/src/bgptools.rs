use super::AsnMap;
use cidr::IpCidr;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Deserialize)]
struct Announcement {
    #[serde(rename = "CIDR")]
    cidr: IpCidr,
    #[serde(rename = "ASN")]
    asn: u32,
}

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

pub fn parse(table: &PathBuf) -> Result<(AsnMap<u32>, AsnMap<u128>), Error> {
    let file = File::open(table)?;
    let reader = BufReader::new(file);

    let mut announcements = Vec::new();
    for line in reader.lines() {
        let announcement: Announcement = serde_json::from_str(&line?)?;
        announcements.push(announcement);
    }

    //let mut map_v4 = BTreeMap::<u32, u32>::new();
    //let mut map_v6 = BTreeMap::<u128, u32>::new();
    let mut map_v4 = BTreeMap::new();
    let mut map_v6 = BTreeMap::new();

    for announcement in announcements {
        match announcement.cidr {
            IpCidr::V4(cidr) => map_v4.insert(cidr.first_address().into(), announcement.asn),
            IpCidr::V6(cidr) => map_v6.insert(cidr.first_address().into(), announcement.asn),
        };
    }

    Ok((AsnMap { map: map_v4 }, AsnMap { map: map_v6 }))
}
