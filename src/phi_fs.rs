//! φ-fs: named storage for quantized φ-memories
//! Save/load sets of φ-encoded data with labels

use std::fs::{File, create_dir_all};
use std::io::{BufWriter, BufReader, Write, Read};
use std::path::{Path, PathBuf};

/// Store for named φ-memories
pub struct PhiMemoryStore {
    base_path: PathBuf,
}

impl PhiMemoryStore {
    /// Create store under given directory
    pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
        let path = base_path.as_ref();
        create_dir_all(path).expect("failed to create φ-store directory");
        Self { base_path: path.to_path_buf() }
    }

    pub fn base_path(&self) -> &std::path::Path {
        &self.base_path
    }
    
    /// Save named φ-data
    pub fn save(&self, name: &str, data: &[f64]) -> std::io::Result<()> {
        let path = self.base_path.join(format!("{}.bin", name));
        let mut writer = BufWriter::new(File::create(path)?);
        for &val in data {
            writer.write_all(&val.to_le_bytes())?;
        }
        Ok(())
    }

    /// Load named φ-data
    pub fn load(&self, name: &str) -> std::io::Result<Vec<f64>> {
        let path = self.base_path.join(format!("{}.bin", name));
        let mut reader = BufReader::new(File::open(path)?);
        let mut buf = [0u8; 8];
        let mut data = Vec::new();
        while reader.read_exact(&mut buf).is_ok() {
            data.push(f64::from_le_bytes(buf));
        }
        Ok(data)
    }

    /// List all stored φ-memory names
    pub fn list(&self) -> std::io::Result<Vec<String>> {
        let mut entries = Vec::new();
        for entry in std::fs::read_dir(&self.base_path)? {
            let entry = entry?;
            if let Some(stem) = entry.path().file_stem() {
                entries.push(stem.to_string_lossy().to_string());
            }
        }
        Ok(entries)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_phi_fs_cycle() {
        let store = PhiMemoryStore::new(".phi_test_store");
        let name = "testwave";
        let original = vec![1.0, 2.0, 3.14];

        store.save(name, &original).unwrap();
        let recovered = store.load(name).unwrap();

        assert_eq!(original.len(), recovered.len());
        for (a, b) in original.iter().zip(recovered.iter()) {
            assert!((a - b).abs() < 1e-12);
        }

        let names = store.list().unwrap();
        assert!(names.contains(&name.to_string()));

        fs::remove_dir_all(".phi_test_store").unwrap();
    }
}
