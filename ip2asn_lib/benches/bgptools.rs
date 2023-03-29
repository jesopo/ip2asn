use cidr::{Ipv4Cidr, Ipv6Cidr};
use criterion::{criterion_group, criterion_main, Criterion};
use ip2asn_lib::AsnMapper as _;
use rand::Rng;
use std::collections::VecDeque;
use std::net::{Ipv4Addr, Ipv6Addr};
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

    let ipv4_cached = Ipv4Cidr::new(Ipv4Addr::from_str("8.8.8.8").unwrap(), 32).unwrap();
    let ipv6_cached =
        Ipv6Cidr::new(Ipv6Addr::from_str("2001:4860:4860::8888").unwrap(), 128).unwrap();
    c.bench_function("v4 cached", |b| b.iter(|| map_v4.lookup(&ipv4_cached)));
    c.bench_function("v6 cached", |b| b.iter(|| map_v6.lookup(&ipv6_cached)));

    let mut rng = rand::thread_rng();

    let mut ipv4_uncached = VecDeque::new();
    let mut ipv6_uncached = VecDeque::new();
    for _ in 0..100_000_000 {
        ipv4_uncached.push_back(Ipv4Cidr::new(rng.gen::<u32>().into(), 32).unwrap());
        ipv6_uncached.push_back(Ipv6Cidr::new(rng.gen::<u128>().into(), 128).unwrap());
    }

    c.bench_function("v4 uncached", |b| {
        b.iter(|| {
            map_v4.lookup(&ipv4_uncached[0]);
            ipv4_uncached.rotate_left(1);
        })
    });
    c.bench_function("v6 uncached", |b| {
        b.iter(|| {
            map_v6.lookup(&ipv6_uncached[0]);
            ipv6_uncached.rotate_left(1);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
