use glam::UVec2;
use indexmap::IndexMap;
use sp_asset::{AssetId, AssetRef};
use sp_math::range::{IRange2, Range2};

#[derive(Clone)]
pub struct AtlasEntryBounds {
    pub rect: IRange2,
    pub norm_rect: Range2,
}

#[derive(Clone)]
pub struct AtlasEntry {
    bounds: Vec<AtlasEntryBounds>,
}

impl AtlasEntry {
    pub fn new(bounds: Vec<AtlasEntryBounds>) -> Self {
        Self { bounds }
    }

    pub fn lod(&self, lod: usize) -> &AtlasEntryBounds {
        &self.bounds[lod]
    }

    pub fn to_norm_rects(&self) -> Vec<Range2> {
        self.bounds.iter().map(|b| b.norm_rect).collect()
    }
}

#[derive(Clone)]
pub struct AtlasDef {
    entries: IndexMap<AssetId, AtlasEntry>,
    size: UVec2,
}

impl AtlasDef {
    pub fn new(size: UVec2, entries: IndexMap<AssetId, AtlasEntry>) -> AtlasDef {
        AtlasDef { size, entries }
    }

    pub fn size(&self) -> UVec2 {
        self.size
    }

    pub fn iter(&self) -> impl Iterator<Item = (&AssetId, &AtlasEntry)> {
        self.entries.iter()
    }

    pub fn get(&self, key: AssetId) -> &AtlasEntry {
        self.entries.get(&key).expect(&format!("No atlas entry for {:?}", key))
    }

    pub fn get_from_ref(&self, r: &AssetRef) -> &AtlasEntry {
        self.entries.get(&r.id).expect(&format!("No atlas entry for {:?}", r.path))
    }

    pub fn rect(&self, id: AssetId) -> IRange2 {
        self.get(id).lod(0).rect
    }

    pub fn norm_rect(&self, id: AssetId) -> Range2 {
        self.get(id).lod(0).norm_rect
    }
} 
