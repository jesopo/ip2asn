use super::common::NetRange;
use cidr::{Cidr, Ipv4Cidr, Ipv6Cidr};
use std::cmp::{Ord, Ordering};
use std::collections::BinaryHeap;

#[derive(Clone, Debug)]
pub(crate) struct Node<T: Cidr> {
    data: NetRange<T::Address>,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    height: u8,
    max_high: T::Address,
}

fn height<T: Cidr>(node: &Option<Box<Node<T>>>) -> i16 {
    node.as_ref().map_or(-1, |n| n.height.into())
}

pub(crate) trait BitCount {
    fn bitcount() -> usize;
}

impl BitCount for Ipv4Cidr {
    fn bitcount() -> usize {
        32
    }
}

impl BitCount for Ipv6Cidr {
    fn bitcount() -> usize {
        128
    }
}

impl<T: Cidr + BitCount> Node<T>
where
    T::Address: std::fmt::Display,
{
    pub fn new(cidr: T, asn: u32) -> Self {
        Self {
            data: NetRange {
                min: cidr.first_address(),
                max: cidr.last_address(),
                len: cidr.network_length(),
                asn,
            },
            left: None,
            right: None,
            max_high: cidr.last_address(),
            height: 0,
        }
    }

    fn _lookup<'a>(
        &'a self,
        key: &T::Address,
        results: &mut BinaryHeap<&'a NetRange<T::Address>>,
        complexity: &mut u8,
    ) {
        *complexity += 1;

        if let Some(left) = &self.left {
            if key <= &left.max_high {
                left._lookup(key, results, complexity);
            }
        }

        if let Some(right) = &self.right {
            if key > &self.data.min && key <= &right.max_high {
                right._lookup(key, results, complexity);
            }
        }

        if key >= &self.data.min && key <= &self.data.max {
            results.push(&self.data);
        }
    }

    pub fn lookup(&self, key: &T::Address) -> (u8, Option<&NetRange<T::Address>>) {
        let mut heap = BinaryHeap::new();
        let mut complexity = 0;
        heap.reserve(T::bitcount());
        self._lookup(key, &mut heap, &mut complexity);
        (complexity, heap.pop())
    }

    pub fn insert(&mut self, node: Node<T>) {
        let which_child = match node
            .data
            .min
            .cmp(&self.data.min)
            .then(node.data.max.cmp(&self.data.max))
        {
            Ordering::Less => &mut self.left,
            Ordering::Greater => &mut self.right,
            _ => unreachable!(
                "duplicate prefix found: {}/{}",
                self.data.min, self.data.len
            ),
        };

        match which_child {
            Some(child) => child.insert(node),
            None => *which_child = Some(Box::new(node)),
        };

        self.update_max_high();
        self.update_height();

        let left_height = height(&self.left);
        let right_height = height(&self.right);

        let balance = left_height - right_height;
        match balance {
            -1 | 0 | 1 => {}
            2 => self.rotate_left_then_right(),
            -2 => self.rotate_right_then_left(),
            _ => unreachable!("bad balance: {balance}"),
        };
    }

    fn update_height(&mut self) {
        let left_height = height(&self.left);
        let right_height = height(&self.right);

        self.height = (std::cmp::max(left_height, right_height) + 1) as u8;
    }

    fn update_max_high(&mut self) {
        let mut max_high = self.data.max;

        if let Some(left) = &self.left {
            if left.max_high > max_high {
                max_high = left.max_high;
            }
        }
        if let Some(right) = &self.right {
            if right.max_high > max_high {
                max_high = right.max_high;
            }
        }

        self.max_high = max_high;
    }

    fn rotate_left(&mut self) {
        let mut right = self.right.take();
        let right_unwrap = right.as_mut().unwrap();
        self.right = right_unwrap.right.take();

        right_unwrap.right = right_unwrap.left.take();
        right_unwrap.left = self.left.take();

        std::mem::swap(&mut self.data, &mut right_unwrap.data);

        right_unwrap.update_height();
        right_unwrap.update_max_high();
        self.left = right;
        self.update_height();
        self.update_max_high();
    }

    fn rotate_right(&mut self) {
        let mut left = self.left.take();
        let left_unwrap = left.as_mut().unwrap();
        self.left = left_unwrap.left.take();

        left_unwrap.left = left_unwrap.right.take();
        left_unwrap.right = self.right.take();

        std::mem::swap(&mut self.data, &mut left_unwrap.data);

        left_unwrap.update_height();
        left_unwrap.update_max_high();
        self.right = left;
        self.update_height();
        self.update_max_high();
    }

    fn rotate_left_then_right(&mut self) {
        let left = self.left.as_mut().unwrap();
        if height(&left.left) < height(&left.right) {
            left.rotate_left();
            self.update_height();
            self.update_max_high();
        }
        self.rotate_right();
    }

    fn rotate_right_then_left(&mut self) {
        let right = self.right.as_mut().unwrap();
        if height(&right.left) > height(&right.right) {
            right.rotate_right();
            self.update_height();
            self.update_max_high();
        }
        self.rotate_left();
    }
}
