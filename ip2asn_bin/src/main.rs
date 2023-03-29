mod util;

use cidr::IpCidr;
use clap::Parser;
use futures::StreamExt as _;
use ip2asn_lib::AsnMapper;
use log::info;
use notify::event::AccessKind;
use notify::{EventKind, RecursiveMode, Watcher as _};
use simplelog::{Config, LevelFilter, SimpleLogger};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr as _;
use std::sync::{Arc, RwLock};
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

    SimpleLogger::init(LevelFilter::Info, Config::default()).unwrap();

    let (map_v4, map_v6) = ip2asn_lib::BgpTools::parse(&args.table).unwrap();
    let map_v4 = Arc::new(RwLock::new(map_v4));
    let map_v6 = Arc::new(RwLock::new(map_v6));

    let route = warp::path::param().map({
        let map_v4 = Arc::clone(&map_v4);
        let map_v6 = Arc::clone(&map_v6);

        move |a: String| {
            let now = std::time::Instant::now();
            let asn = match IpCidr::from_str(&a).unwrap() {
                IpCidr::V4(cidr) => map_v4.read().unwrap().lookup(&cidr).cloned(),
                IpCidr::V6(cidr) => map_v6.read().unwrap().lookup(&cidr).cloned(),
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
        }
    });

    let (mut watcher, mut rx) = self::util::async_watcher().unwrap();
    watcher
        .watch(&args.table, RecursiveMode::NonRecursive)
        .unwrap();

    let map_v4 = Arc::clone(&map_v4);
    let map_v6 = Arc::clone(&map_v6);

    tokio::join!(warp::serve(route).run(args.bind), async move {
        while let Some(res) = rx.next().await {
            if let Ok(res) = res {
                if let EventKind::Access(AccessKind::Close(_)) = res.kind {
                    match ip2asn_lib::BgpTools::parse(&args.table) {
                        Ok((new_map_v4, new_map_v6)) => {
                            *map_v4.write().unwrap() = new_map_v4;
                            *map_v6.write().unwrap() = new_map_v6;
                            info!("reloaded table");
                        }
                        Err(e) => {
                            eprintln!("couldn't load table: {e:?}");
                        }
                    };
                }
            }
        }
    });
}
