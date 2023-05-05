pub mod bgptools;
mod common;
mod interval_tree;

pub use self::bgptools::BgpTools;
pub use self::common::{Error, NetRange};
use self::interval_tree::Node;
use cidr::{Cidr, Ipv4Cidr, Ipv6Cidr};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::path::Path;

pub trait AsnMap<T: Cidr> {
    fn new() -> Self;
    fn insert(&mut self, cidr: T, asn: u32);
    fn lookup(&self, address: &T::Address) -> (u8, Option<&NetRange<T::Address>>);
}

pub struct Ipv4AsnMap {
    node: Option<Node<Ipv4Cidr>>,
}

impl AsnMap<Ipv4Cidr> for Ipv4AsnMap {
    fn new() -> Self {
        Self { node: None }
    }

    fn insert(&mut self, cidr: Ipv4Cidr, asn: u32) {
        let new_node = Node::new(cidr, asn);
        if let Some(root) = &mut self.node {
            root.insert(new_node);
        } else {
            self.node = Some(new_node);
        }
    }

    fn lookup(&self, address: &Ipv4Addr) -> (u8, Option<&NetRange<Ipv4Addr>>) {
        if let Some(node) = &self.node {
            node.lookup(address)
        } else {
            (0, None)
        }
    }
}

pub struct Ipv6AsnMap {
    node: Option<Node<Ipv6Cidr>>,
}

impl AsnMap<Ipv6Cidr> for Ipv6AsnMap {
    fn new() -> Self {
        Self { node: None }
    }

    fn insert(&mut self, cidr: Ipv6Cidr, asn: u32) {
        let new_node = Node::new(cidr, asn);
        if let Some(root) = &mut self.node {
            root.insert(new_node);
        } else {
            self.node = Some(new_node);
        }
    }

    fn lookup(&self, address: &Ipv6Addr) -> (u8, Option<&NetRange<Ipv6Addr>>) {
        if let Some(node) = &self.node {
            node.lookup(address)
        } else {
            (0, None)
        }
    }
}

pub trait AsnMapper {
    /// Take a path to an IP<->ASN mapping file, read it, and parse it.
    ///
    /// # Errors
    ///
    /// Will return `Err` if there was an issue reading or parsing IP<->ASN
    /// mapping file.
    fn parse(path: &Path) -> Result<(Ipv4AsnMap, Ipv6AsnMap), Error>;
}
