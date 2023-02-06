use std::path::Path;

fn canonicalize_path(path: &Path) -> String {
    path.to_string_lossy().to_string().replace('\\', "/")
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AssetId(pub u64);

impl AssetId {
    // fnv1a64
    const PRIME: u64 = 0x100000001b3;
    const SEED: u64 = 0xcbf29ce484222325;

    const fn combine(hash: u64, value: u64) -> u64 {
        (hash ^ value).wrapping_mul(Self::PRIME)
    }

    const fn hash(value: &[u8]) -> u64 {
        let mut h = Self::SEED;
        let mut i = 0;
        while i < value.len() {
            h = Self::combine(h, value[i] as u64);
            i += 1;
        }
        h
    }

    pub const fn from_str(input: &str) -> Self {
        Self(Self::hash(input.as_bytes()))
    }

    pub fn from_path(path: &Path) -> Self {
        Self::from_str(&canonicalize_path(path))
    }
}

#[derive(Debug, Clone)]
pub struct AssetRef {
    pub path: String,
    pub id: AssetId,
}

impl Default for AssetRef {
    fn default() -> Self {
        Self::from_str("")
    }
}

impl AssetRef {
    pub fn from_str(input: &str) -> Self {
        Self {
            path: input.to_string(),
            id: AssetId::from_str(input),
        }
    }

    pub fn update_id(&mut self) {
        self.id = AssetId::from_str(&self.path);
    }

    pub fn from_path(path: &Path) -> Self {
        Self::from_str(&canonicalize_path(path))
    }
}
