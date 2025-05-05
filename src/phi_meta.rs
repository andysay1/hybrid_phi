//! phi_meta: metadata handler for φ-memory entries
//! Supports structured save/load of metadata alongside binary φ-data

use std::fs::File;
use std::io::{BufWriter, BufReader, Write, BufRead};
use std::path::Path;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhiMetadata {
    pub n: usize,
    pub step: f64,
    pub length: usize,
    pub saved_at: DateTime<Utc>,
}

impl PhiMetadata {
    /// Save metadata to .meta.txt file
    pub fn save<P: AsRef<Path>>(&self, name: &str, base_path: P) -> std::io::Result<()> {
        let path = base_path.as_ref().join(format!("{}.meta.txt", name));
        let mut writer = BufWriter::new(File::create(path)?);
        writeln!(writer, "n={}", self.n)?;
        writeln!(writer, "step={:.6}", self.step)?;
        writeln!(writer, "length={}", self.length)?;
        writeln!(writer, "saved_at={}", self.saved_at.to_rfc3339())?;
        Ok(())
    }

    /// Load metadata from .meta.txt file
    pub fn load<P: AsRef<Path>>(name: &str, base_path: P) -> std::io::Result<Self> {
        let path = base_path.as_ref().join(format!("{}.meta.txt", name));
        let file = BufReader::new(File::open(path)?);

        let mut n = 0;
        let mut step = 0.0;
        let mut length = 0;
        let mut saved_at = Utc::now();

        for line in file.lines() {
            let line = line?;
            if let Some(v) = line.strip_prefix("n=") {
                n = v.parse().unwrap_or(0);
            } else if let Some(v) = line.strip_prefix("step=") {
                step = v.parse().unwrap_or(0.0);
            } else if let Some(v) = line.strip_prefix("length=") {
                length = v.parse().unwrap_or(0);
            } else if let Some(v) = line.strip_prefix("saved_at=") {
                saved_at = DateTime::parse_from_rfc3339(v)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or(Utc::now());
            }
        }

        Ok(Self { n, step, length, saved_at })
    }
}