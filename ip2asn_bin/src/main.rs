mod util;

use clap::Parser;
use futures::StreamExt as _;
use ip2asn_lib::{AsnMap, AsnMapper};
use log::info;
use notify::event::AccessKind;
use notify::{EventKind, RecursiveMode, Watcher as _};
use simplelog::{Config, LevelFilter, SimpleLogger};
use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;
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
    #[clap(long, short, action)]
    verbose: bool,
}

fn micros_float(duration: &Duration) -> String {
    let nanos = duration.as_nanos();
    format!("{}.{:03}", nanos / 1000, nanos % 1000)
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    SimpleLogger::init(
        match args.verbose {
            true => LevelFilter::Debug,
            false => LevelFilter::Info,
        },
        Config::default(),
    )
    .unwrap();

    let (map_v4, map_v6) = ip2asn_lib::BgpTools::parse(&args.table).unwrap();
    let map_v4 = Arc::new(RwLock::new(map_v4));
    let map_v6 = Arc::new(RwLock::new(map_v6));

    let route = warp::path::param().map({
        let map_v4 = Arc::clone(&map_v4);
        let map_v6 = Arc::clone(&map_v6);

        move |ip: IpAddr| {
            let now = std::time::Instant::now();
            let (complexity, asn) = match ip {
                IpAddr::V4(ip) => {
                    let map = map_v4.read().unwrap();
                    let (complexity, range) = map.lookup(&ip);
                    (complexity, range.map(|r| r.asn))
                }
                IpAddr::V6(ip) => {
                    let map = map_v6.read().unwrap();
                    let (complexity, range) = map.lookup(&ip);
                    (complexity, range.map(|r| r.asn))
                }
            };

            let (status, body) = match asn {
                Some(asn) => (200, asn.to_string()),
                None => (404, String::new()),
            };

            warp::http::Response::builder()
                .status(status)
                .header("X-Elapsed", format!("{}µs", micros_float(&now.elapsed())))
                .header("X-Complexity", complexity.to_string())
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
