use md5::Context;
use sha2::{Sha256, Sha512, Digest};
use clap::ValueEnum;

fn to_hex(data: &[u8]) -> String {
    let mut out = String::new();
    for x in data {
        out += &format!("{:x}", x);
    }
    out
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Algorithm {
    Sha256,
    Sha512,
    Md5
}


pub trait DigestAlgorithm {
    fn finalize(&mut self) -> String;
    fn update(&mut self, data: &[u8]);
}

pub struct Sha256Algorithm{
    hasher: Sha256,
}

pub struct Sha512Algorithm{
    hasher: Sha512,
}

pub struct Md5Algorithm{
    context: Context,
}

impl DigestAlgorithm for Md5Algorithm {
    fn update(&mut self, data: &[u8]){
        self.context.consume(data);
    }

    fn finalize(&mut self) -> String{
        to_hex(self.context.clone().compute().as_slice())
    }
}

impl DigestAlgorithm for Sha256Algorithm {
    fn update(&mut self, data: &[u8]){
        self.hasher.update(data);
    }

    fn finalize(&mut self) -> String{
        to_hex(&self.hasher.clone().finalize())
    }
}

impl DigestAlgorithm for Sha512Algorithm {
    fn update(&mut self, data: &[u8]){
        self.hasher.update(data);
    }

    fn finalize(&mut self) -> String{
        to_hex(&self.hasher.clone().finalize())
    }
}

impl Sha256Algorithm {
    pub fn new() -> Self {
        Sha256Algorithm {
            hasher: Sha256::new()
        }
    }
}

impl Sha512Algorithm {
    pub fn new() -> Self {
        Sha512Algorithm {
            hasher: Sha512::new()
        }
    }
}

impl Md5Algorithm {
    pub fn new() -> Self {
        Md5Algorithm {
            context: Context::new()
        }
    }
}
