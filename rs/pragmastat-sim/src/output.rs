use crate::sim::SimulationRow;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

/// Thread-safe incremental JSON writer backed by a BTreeMap.
pub struct OutputWriter<V: SimulationRow> {
    path: PathBuf,
    rows: Mutex<BTreeMap<String, V>>,
}

impl<V: SimulationRow> OutputWriter<V> {
    pub fn new(path: PathBuf, existing: BTreeMap<String, V>) -> Self {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).ok();
        }
        Self {
            path,
            rows: Mutex::new(existing),
        }
    }

    /// Insert or update a row and flush to disk.
    pub fn write_row(&self, key: String, row: V) {
        {
            let mut map = self.rows.lock().unwrap();
            map.insert(key, row);
        }
        self.flush();
    }

    /// Final flush; returns the output path.
    pub fn finalize(&self) -> &std::path::Path {
        self.flush();
        &self.path
    }

    fn flush(&self) {
        let mut rows: Vec<V> = {
            let map = self.rows.lock().unwrap();
            map.values().cloned().collect()
        };
        rows.sort();
        let json = serde_json::to_string_pretty(&rows).expect("JSON serialization failed");
        fs::write(&self.path, json).expect("Failed to write results file");
    }
}
