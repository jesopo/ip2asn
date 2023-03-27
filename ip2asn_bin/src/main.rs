use clap::Parser;
use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr as _;
use std::sync::Arc;
use std::time::Duration;
use warp::Filter as _;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(index = 1)]
    table: PathBuf,
    #[clap(index = 2)]
    bind: SocketAddr,
}

fn micros_float(duration: &Duration) -> String {
    let nanos = duration.as_nanos();
    format!("{}.{:03}", nanos / 1000, nanos % 1000)
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let (map_v4, map_v6) = ip2asn_lib::bgptools::parse(&args.table).unwrap();
    let map_v4 = Arc::new(map_v4);
    let map_v6 = Arc::new(map_v6);

    let route = warp::path::param().map(move |a: String| {
        let now = std::time::Instant::now();
        let asn = match IpAddr::from_str(&a).unwrap() {
            IpAddr::V4(ip) => map_v4.lookup(ip),
            IpAddr::V6(ip) => map_v6.lookup(ip),
        };

        let builder = warp::http::Response::builder();

        let (status, body) = match asn {
            Some(asn) => (200, asn.to_string()),
            None => (404, String::new()),
        };

        builder
            .status(status)
            .header("X-Elapsed", format!("{}µs", micros_float(&now.elapsed())))
            .body(body)
    });

    warp::serve(route).run(args.bind).await;
}
