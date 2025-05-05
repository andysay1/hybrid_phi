//! phi_bundle: self-contained φ-memory package (data + metadata)
//! Supports JSON serialization for portable storage and transmission

use crate::phi_fs::PhiMemoryStore;
use crate::phi_meta::PhiMetadata;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct PhiBundle {
    pub name: String,
    pub metadata: PhiMetadata,
    pub data: Vec<f64>,
}

impl PhiBundle {
    /// Construct bundle from memory store
    pub fn from_store(name: &str, store: &PhiMemoryStore) -> std::io::Result<Self> {
        let data = store.load(name)?;
        let metadata = PhiMetadata::load(name, store.base_path())?;
        Ok(Self {
            name: name.to_string(),
            metadata,
            data,
        })
    }

    /// Save bundle to JSON file
    pub fn save_json<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self).unwrap();
        fs::write(path, json)
    }

    /// Load bundle from JSON file
    pub fn load_json<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let json = fs::read_to_string(path)?;
        let bundle: Self = serde_json::from_str(&json).unwrap();
        Ok(bundle)
    }

    /// Restore to memory store
    pub fn save_to_store(&self, store: &PhiMemoryStore) -> std::io::Result<()> {
        store.save(&self.name, &self.data)?;
        self.metadata.save(&self.name, store.base_path())
    }
}
