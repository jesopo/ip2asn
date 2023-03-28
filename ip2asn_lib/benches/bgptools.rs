use std::net::{Ipv4Addr, Ipv6Addr};
use criterion::{criterion_group, criterion_main, Criterion};
use ip2asn_lib::AsnMapper as _;
use rand::Rng;
use std::path::PathBuf;
use std::str::FromStr as _;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("load", |b| {
        b.iter(|| {
            ip2asn_lib::BgpTools::parse(&PathBuf::from_str("benches/bgptools.jsonl").unwrap())
                .unwrap()
        })
    });

    let (map_v4, map_v6) =
        ip2asn_lib::BgpTools::parse(&PathBuf::from_str("benches/bgptools.jsonl").unwrap()).unwrap();

    let ipv4_cached = Ipv4Addr::from_str("8.8.8.8").unwrap();
    let ipv6_cached = Ipv6Addr::from_str("2001:4860:4860::8888").unwrap();
    c.bench_function("v4 cached", |b| b.iter(|| map_v4.lookup(ipv4_cached)));
    c.bench_function("v6 cached", |b| b.iter(|| map_v6.lookup(ipv6_cached)));

    let mut rng = rand::thread_rng();
    c.bench_function("v4 uncached", |b| b.iter(|| map_v4.lookup(rng.gen::<u32>())));
    c.bench_function("v6 uncached", |b| b.iter(|| map_v6.lookup(rng.gen::<u128>())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
