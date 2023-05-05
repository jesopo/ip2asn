use super::{AsnMap as _, AsnMapper, Error, Ipv4AsnMap, Ipv6AsnMap};
use cidr::IpCidr;
use log::info;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct Announcement {
    #[serde(rename = "CIDR")]
    cidr: IpCidr,
    #[serde(rename = "ASN")]
    asn: u32,
    #[serde(rename = "Hits")]
    hits: u64,
}

pub struct BgpTools {}

impl AsnMapper for BgpTools {
    fn parse(table: &Path) -> Result<(Ipv4AsnMap, Ipv6AsnMap), Error> {
        let file = File::open(table)?;
        let reader = BufReader::new(file);

        let mut announcements = BTreeMap::new();
        for line in reader.lines() {
            let line = line?;
            if line.starts_with("//") {
                continue;
            }

            let announcement: Announcement = serde_json::from_str(&line)?;

            if let Some((_, hits)) = announcements.get(&announcement.cidr) {
                if hits > &announcement.hits {
                    continue;
                }
            }

            announcements.insert(announcement.cidr, (announcement.asn, announcement.hits));
        }

        let mut i_v4 = 0;
        let mut map_v4 = Ipv4AsnMap::new();
        let mut i_v6 = 0;
        let mut map_v6 = Ipv6AsnMap::new();

        for (cidr, (asn, _)) in announcements.into_iter() {
            match cidr {
                IpCidr::V4(cidr) => {
                    i_v4 += 1;
                    map_v4.insert(cidr, asn);
                }
                IpCidr::V6(cidr) => {
                    i_v6 += 1;
                    map_v6.insert(cidr, asn);
                }
            };
        }

        info!("loaded {i_v4} v4 & {i_v6} v6");
        Ok((map_v4, map_v6))
    }
}
