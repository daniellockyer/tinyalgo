#![no_std]

const DELTA: u32 = 0x9e3779b9;
const SUMMATION: u32 = 0xc6ef3720;
const ROUNDS: u32 = 32;

pub fn encrypt(block: &mut [u32; 2], key: [u32; 4]) -> &mut [u32; 2] {
    let mut sum: u32 = 0;
    let (k0, k1, k2, k3) = (key[0], key[1], key[2], key[3]);

    for _ in 0..ROUNDS {
        sum = sum.wrapping_add(DELTA);
        block[0] = block[0].wrapping_add((block[1] << 4).wrapping_add(k0) ^ block[1].wrapping_add(sum) ^ (block[1] >> 5).wrapping_add(k1));
        block[1] = block[1].wrapping_add((block[0] << 4).wrapping_add(k2) ^ block[0].wrapping_add(sum) ^ (block[0] >> 5).wrapping_add(k3));
    }
    block
}

pub fn decrypt(block: &mut [u32; 2], key: [u32; 4]) -> &mut [u32; 2] {
    let mut sum = SUMMATION;
    let (k0, k1, k2, k3) = (key[0], key[1], key[2], key[3]);

    for _ in 0..ROUNDS {
        block[1] = block[1].wrapping_sub((block[0] << 4).wrapping_add(k2) ^ block[0].wrapping_add(sum) ^ (block[0] >> 5).wrapping_add(k3));
        block[0] = block[0].wrapping_sub((block[1] << 4).wrapping_add(k0) ^ block[1].wrapping_add(sum) ^ (block[1] >> 5).wrapping_add(k1));
        sum = sum.wrapping_sub(DELTA);
    }
    block
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let block = &mut [1,2];
        let key = [1,2,3,4];

        let a = encrypt(block, key);
        let b = decrypt(a, key);
        assert_eq!(b, &mut [1,2]);
    }
}
