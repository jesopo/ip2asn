pub mod bgptools;

use std::collections::BTreeMap;

pub struct AsnMap<T: std::cmp::Ord> {
    map: BTreeMap<T, u32>,
}

impl<T: std::cmp::Ord> AsnMap<T> {
    pub fn lookup(&self, key: impl Into<T>) -> Option<&u32> {
        self.map.range(..=key.into()).next_back().map(|(_, v)| v)
    }
}
