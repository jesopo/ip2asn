use cidr::{Ipv4Cidr, Ipv6Cidr};
use criterion::{criterion_group, criterion_main, Criterion};
use ip2asn_lib::AsnMapper as _;
use rand::Rng;
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
    c.bench_function("v4 uncached", |b| {
        b.iter(|| {
            let cidr = Ipv4Cidr::new(rng.gen::<u32>().into(), 32).unwrap();
            map_v4.lookup(&cidr);
        })
    });
    c.bench_function("v6 uncached", |b| {
        b.iter(|| {
            let cidr = Ipv6Cidr::new(rng.gen::<u128>().into(), 128).unwrap();
            map_v6.lookup(&cidr);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
