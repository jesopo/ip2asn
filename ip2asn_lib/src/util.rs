use cidr::{Ipv4Cidr, Ipv6Cidr};
use std::net::{Ipv4Addr, Ipv6Addr};

pub trait Bytekey {
    fn get_bytekey(&self) -> Vec<u8>;
}

impl Bytekey for Ipv4Addr {
    fn get_bytekey(&self) -> Vec<u8> {
        Into::<u32>::into(*self).to_be_bytes().into()
    }
}

impl Bytekey for Ipv6Addr {
    fn get_bytekey(&self) -> Vec<u8> {
        Into::<u128>::into(*self).to_be_bytes().into()
    }
}

impl Bytekey for Ipv4Cidr {
    fn get_bytekey(&self) -> Vec<u8> {
        let mut bytes = Into::<u32>::into(self.first_address())
            .to_be_bytes()
            .to_vec();

        let mut byte_count = self.network_length() / 8;
        if self.network_length() % 8 > 0 {
            byte_count += 1;
        }

        bytes.truncate(byte_count as usize);
        bytes
    }
}

impl Bytekey for Ipv6Cidr {
    fn get_bytekey(&self) -> Vec<u8> {
        let mut bytes = Into::<u128>::into(self.first_address())
            .to_be_bytes()
            .to_vec();

        let mut byte_count = self.network_length() / 8;
        if self.network_length() % 8 > 0 {
            byte_count += 1;
        }

        bytes.truncate(byte_count as usize);
        bytes
    }
}
