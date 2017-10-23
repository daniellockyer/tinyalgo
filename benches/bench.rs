#![feature(test)]
extern crate test;
extern crate tinyalgo;

use test::Bencher;
use tinyalgo::{encrypt, decrypt};

#[bench]
fn empty(b: &mut Bencher) {
    let block = &mut [1,2];
    let key = [1,2,3,4];

    b.iter(|| {
        let a = encrypt(block, key);
        let b = decrypt(a, key);
    })
}

