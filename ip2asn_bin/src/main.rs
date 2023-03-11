use cidr::IpCidr;
use clap;
use clap::Parser;
use ip2asn_lib::Node;
use serde::Deserialize;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::net::IpAddr;
use std::path::PathBuf;
use std::str::FromStr as _;
use std::sync::Arc;
use warp::Filter as _;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(index = 1)]
    table: PathBuf,
    #[clap(index = 2)]
    port: u16,
}

#[derive(Deserialize)]
struct Announcement {
    #[serde(rename = "CIDR")]
    cidr: IpCidr,
    #[serde(rename = "ASN")]
    asn: u32,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let file = File::open(args.table).unwrap();
    let reader = BufReader::new(file);

    let mut announcements = Vec::new();
    let now = std::time::Instant::now();
    for line in reader.lines() {
        let announcement: Announcement = serde_json::from_str(&line.unwrap()).unwrap();
        announcements.push(announcement);
    }
    println!("read json in {}s", now.elapsed().as_secs_f64());

    let mut trie_v4 = Node::new();
    let mut trie_v6 = Node::new();

    let now = std::time::Instant::now();
    for announcement in announcements {
        match announcement.cidr {
            IpCidr::V4(cidr) => trie_v4.insert(&cidr, announcement.asn),
            IpCidr::V6(cidr) => trie_v6.insert(&cidr, announcement.asn),
        };
    }
    println!("load trie in {}s", now.elapsed().as_secs_f64());

    let trie_v4 = Arc::new(trie_v4);
    let trie_v6 = Arc::new(trie_v6);

    let route = warp::path::param().map(move |a: String| {
        let now = std::time::Instant::now();
        let (asn, depth) = match IpAddr::from_str(&a).unwrap() {
            IpAddr::V4(ip) => trie_v4.lookup(&ip),
            IpAddr::V6(ip) => trie_v6.lookup(&ip),
        };

        let builder = warp::http::Response::builder();

        let (status, body) = match asn {
            Some(asn) => (200, asn.to_string()),
            None => (404, "".to_string()),
        };

        builder
            .status(status)
            .header("X-Depth", depth.to_string())
            .header(
                "X-Elapsed",
                format!("{}µs", (now.elapsed().as_nanos() as f64) / 1000.0),
            )
            .body(body)
    });

    warp::serve(route).run(([10, 84, 1, 1], args.port)).await;
}
