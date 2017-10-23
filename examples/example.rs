#![feature(alloc_system)] extern crate alloc_system;
extern crate tinyalgo;

fn main() {
    let block = &mut [1,2];
    let key = [1,2,3,4];

    let a = tinyalgo::encrypt(block, key);
    let b = tinyalgo::decrypt(a, key);
    assert_eq!(b, &mut [1,2]);
}
