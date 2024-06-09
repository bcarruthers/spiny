use std::num::Wrapping;

use glam::{IVec2, IVec3};

const _PRIME32_1: Wrapping<u32> = Wrapping(2654435761u32);
const PRIME32_2: Wrapping<u32> = Wrapping(2246822519u32);
const PRIME32_3: Wrapping<u32> = Wrapping(3266489917u32);
const PRIME32_4: Wrapping<u32> = Wrapping(668265263u32);
const PRIME32_5: Wrapping<u32> = Wrapping(374761393u32);

fn rotate_left(value: u32, count: u32) -> u32 {
    (value << count) | (value >> (32 - count))
}

pub fn finalize(hash: u32) -> u32 {
    let h = Wrapping(hash);
    let h = h ^ (h >> 15);
    let h = h * PRIME32_2;
    let h = h ^ (h >> 13);
    let h = h * PRIME32_3;
    let h = h ^ (h >> 16);
    h.0
}

pub fn finalize_in(hash: u32, index: u32, min: u32, max: u32) -> u32 {
    let h = combine(hash, index);
    let h = finalize(h);
    (h % (max - min)) + min
}

pub fn combine(hash: u32, value: u32) -> u32 {
    let h = Wrapping(hash) + Wrapping(value) * PRIME32_3;
    let h = Wrapping(rotate_left(h.0, 17)) * PRIME32_4;
    h.0
}

pub fn combine_u64(hash: u32, value: u64) -> u32 {
    let h = combine(hash, (value & 0xffffffff) as u32);
    let h = combine(h, (value >> 32) as u32);
    h
}

pub fn init(seed: u32) -> u32 {
    (Wrapping(seed) + PRIME32_5).0
}

pub fn init_size(seed: u32, size: u32) -> u32 {
    let h = init(seed);
    let h = Wrapping(h) + Wrapping(size);
    h.0
}

pub fn hash_u32(seed: u32, value: u32) -> u32 {
    let h = init_size(seed, 4);
    let h = combine(h, value);
    finalize(h)
}

pub fn hash2_u32(seed: u32, x1: u32, x2: u32) -> u32 {
    let h = init_size(seed, 8);
    let h = combine(h, x1);
    let h = combine(h, x2);
    finalize(h)
}

pub fn hash3_u32(seed: u32, x1: u32, x2: u32, x3: u32) -> u32 {
    let h = init_size(seed, 12);
    let h = combine(h, x1);
    let h = combine(h, x2);
    let h = combine(h, x3);
    finalize(h)
}

pub fn hash_i32(seed: u32, value: i32) -> u32 {
    hash_u32(seed, value as u32)
}

pub fn hash2_i32(seed: u32, x1: i32, x2: i32) -> u32 {
    hash2_u32(seed, x1 as u32, x2 as u32)
}

pub fn hash3_i32(seed: u32, x1: i32, x2: i32, x3: i32) -> u32 {
    hash3_u32(seed, x1 as u32, x2 as u32, x3 as u32)
}

pub fn hash_ivec2(seed: u32, v: IVec2) -> u32 {
    hash2_i32(seed, v.x, v.y)
}

pub fn hash_ivec3(seed: u32, v: IVec3) -> u32 {
    hash3_i32(seed, v.x, v.y, v.z)
}