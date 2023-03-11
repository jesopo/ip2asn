mod util;

use self::util::Bytekey;
use std::collections::BTreeMap;

#[derive(Default)]
pub struct Node {
    data: Option<u32>,
    kids: BTreeMap<u8, Box<Node>>,
}

impl Node {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    fn insert_bytekey(&mut self, key: &[u8], data: u32) {
        let kid = self.kids.entry(key[0]).or_insert_with(Box::<Node>::default);

        if key.len() > 1 {
            kid.insert_bytekey(&key[1..], data);
        } else {
            kid.data = Some(data);
        }
    }

    pub fn insert(&mut self, key: &impl Bytekey, data: u32) {
        self.insert_bytekey(&key.get_bytekey(), data);
    }

    fn lookup_bytekey(&self, key: &[u8], depth: u8) -> (Option<u32>, u8) {
        if !key.is_empty() {
            if let Some(kid) = self.kids.get(&key[0]) {
                return kid.lookup_bytekey(&key[1..], depth + 1);
            }
        }

        (self.data, depth)
    }

    pub fn lookup(&self, key: &impl Bytekey) -> (Option<u32>, u8) {
        self.lookup_bytekey(&key.get_bytekey(), 0)
    }
}
