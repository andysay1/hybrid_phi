//! φ-storage module: save/load quantized φ-memory to/from file

use std::fs::File;
use std::io::{BufReader, BufWriter, Write, Read};
use std::path::Path;

/// Save a vector of quantized φ-values to a binary file
pub fn save_quantized<P: AsRef<Path>>(data: &[f64], path: P) -> std::io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    for &val in data {
        writer.write_all(&val.to_le_bytes())?;
    }
    Ok(())
}

/// Load a vector of quantized φ-values from a binary file
pub fn load_quantized<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<f64>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buf = [0u8; 8];
    let mut data = Vec::new();

    while reader.read_exact(&mut buf).is_ok() {
        let val = f64::from_le_bytes(buf);
        data.push(val);
    }
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_save_load_quantized() {
        let path = "test_quantized.bin";
        let original = vec![-1.0, 0.0, 0.5, 1.0, 123.456];

        save_quantized(&original, path).expect("save failed");
        let loaded = load_quantized(path).expect("load failed");

        fs::remove_file(path).unwrap();

        assert_eq!(original.len(), loaded.len());
        for (a, b) in original.iter().zip(loaded.iter()) {
            assert!((a - b).abs() < 1e-12, "a = {}, b = {}", a, b);
        }
    }
}