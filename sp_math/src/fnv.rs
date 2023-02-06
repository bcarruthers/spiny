pub mod fnv1a {
    use std::num::Wrapping;

    pub const PRIME: u32 = 0x1000193;
    pub const SEED: u32 = 0x811c9dc5;

    pub fn combine(hash: u32, value: u32) -> u32 {
        (Wrapping(hash ^ value) * Wrapping(PRIME)).0
    }

    pub fn hash(x1: u32) -> u32 {
        let h = SEED;
        let h = combine(h, x1);
        h
    }

    pub fn hash2(x1: u32, x2: u32) -> u32 {
        let h = SEED;
        let h = combine(h, x1);
        let h = combine(h, x2);
        h
    }

    pub fn hash3(x1: u32, x2: u32, x3: u32) -> u32 {
        let h = SEED;
        let h = combine(h, x1);
        let h = combine(h, x2);
        let h = combine(h, x3);
        h
    }
}

pub mod fnv1a64 {
    use std::num::Wrapping;

    pub const PRIME: u64 = 0x100000001b3;
    pub const SEED: u64 = 0xcbf29ce484222325;

    pub fn combine(hash: u64, value: u64) -> u64 {
        (Wrapping(hash ^ value) * Wrapping(PRIME)).0
    }

    pub fn hash(value: u64) -> u64 {
        let h = SEED;
        let h = combine(h, value);
        h
    }

    pub fn hash2(x1: u64, x2: u64) -> u64 {
        let h = SEED;
        let h = combine(h, x1);
        let h = combine(h, x2);
        h
    }

    pub fn hash3(x1: u64, x2: u64, x3: u64) -> u64 {
        let h = SEED;
        let h = combine(h, x1);
        let h = combine(h, x2);
        let h = combine(h, x3);
        h
    }
}
