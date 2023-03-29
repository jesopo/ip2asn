use super::{AsnMap, AsnMapper, Error};
use cidr::{IpCidr, Ipv4Cidr, Ipv6Cidr};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize)]
struct Announcement {
    #[serde(rename = "CIDR")]
    cidr: IpCidr,
    #[serde(rename = "ASN")]
    asn: u32,
}

pub struct BgpTools {}

impl AsnMapper for BgpTools {
    fn parse(table: &Path) -> Result<(AsnMap<Ipv4Cidr>, AsnMap<Ipv6Cidr>), Error> {
        let file = File::open(table)?;
        let reader = BufReader::new(file);

        let mut map_v4 = BTreeMap::new();
        let mut map_v6 = BTreeMap::new();

        for line in reader.lines() {
            let line = line?;
            if line.starts_with("//") {
                continue;
            }

            let announcement: Announcement = serde_json::from_str(&line)?;
            match announcement.cidr {
                IpCidr::V4(cidr) => map_v4.insert(cidr, announcement.asn),
                IpCidr::V6(cidr) => map_v6.insert(cidr, announcement.asn),
            };
        }

        Ok((AsnMap { map: map_v4 }, AsnMap { map: map_v6 }))
    }
}
