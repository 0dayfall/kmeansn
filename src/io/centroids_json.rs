use std::io::{Read, Write};

use serde::{Deserialize, Serialize};

use crate::algo::kmeans::FitResult;
use crate::model::{Centroid, Dataset};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CentroidsFile {
    pub version: String,
    pub dims: usize,
    pub columns: Vec<String>,
    pub distance: String,
    pub centroids: Vec<Centroid>,
    pub data_count: usize,
    pub converged: bool,
    pub iterations: usize,
}

impl CentroidsFile {
    pub fn from_fit(dataset: &Dataset, fit: &FitResult) -> Self {
        Self {
            version: "kmeansn.centroids.v1".to_string(),
            dims: dataset.columns.len(),
            columns: dataset.columns.clone(),
            distance: "euclidean".to_string(),
            centroids: fit.centroids.clone(),
            data_count: fit.data_count,
            converged: fit.converged,
            iterations: fit.iterations,
        }
    }

    pub fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.columns.is_empty() || self.dims == 0 {
            return Err("centroids file has no feature columns".into());
        }
        if self.dims != self.columns.len() {
            return Err("centroids dims does not match columns length".into());
        }
        for centroid in &self.centroids {
            if centroid.coords.len() != self.dims {
                return Err("centroid coords length does not match dims".into());
            }
        }
        Ok(())
    }
}

pub fn read_centroids<R: Read>(reader: R) -> Result<CentroidsFile, Box<dyn std::error::Error>> {
    let file: CentroidsFile = serde_json::from_reader(reader)?;
    file.validate()?;
    Ok(file)
}

pub fn write_centroids<W: Write>(
    mut writer: W,
    file: &CentroidsFile,
) -> Result<(), Box<dyn std::error::Error>> {
    serde_json::to_writer_pretty(&mut writer, file)?;
    writeln!(writer)?;
    Ok(())
}
