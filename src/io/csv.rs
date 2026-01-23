use std::collections::HashSet;
use std::io::{Read, Write};

use csv::{ReaderBuilder, WriterBuilder};

use crate::algo::neighbors::Neighbor;
use crate::model::{Dataset, Point};
use crate::output::assign::AssignedPoint;

fn is_reserved(name: &str) -> bool {
    name == "id" || name.starts_with('_')
}

pub fn read_points<R: Read>(
    reader: R,
    dataset: &mut Dataset,
) -> Result<Vec<Point>, Box<dyn std::error::Error>> {
    let mut csv = ReaderBuilder::new().has_headers(true).from_reader(reader);
    let headers = csv.headers()?.clone();

    let header_features: Vec<String> = headers
        .iter()
        .filter(|name| !is_reserved(name))
        .map(|name| name.to_string())
        .collect();

    if dataset.columns.is_empty() {
        dataset.columns = header_features.clone();
    } else {
        let expected: HashSet<&str> = dataset.columns.iter().map(|s| s.as_str()).collect();
        let actual: HashSet<&str> = header_features.iter().map(|s| s.as_str()).collect();
        if expected != actual {
            return Err("input columns do not match centroids columns".into());
        }
    }

    let mut indices = Vec::with_capacity(dataset.columns.len());
    for col in &dataset.columns {
        let pos = headers
            .iter()
            .position(|name| name == col)
            .ok_or_else(|| "missing required column".to_string())?;
        indices.push(pos);
    }

    let id_index = headers.iter().position(|name| name == "id");
    dataset.has_id = id_index.is_some();

    let mut points = Vec::new();
    for record in csv.records() {
        let record = record?;
        let id = id_index
            .and_then(|idx| record.get(idx))
            .and_then(|s| if s.is_empty() { None } else { Some(s.to_string()) });

        let mut coords = Vec::with_capacity(indices.len());
        for &idx in &indices {
            let raw = record.get(idx).ok_or_else(|| "missing field".to_string())?;
            let value: f64 = raw.parse()?;
            coords.push(value);
        }

        points.push(Point { id, coords });
    }

    Ok(points)
}

pub fn write_assigned_points<W: Write>(
    writer: W,
    dataset: &Dataset,
    points: &[AssignedPoint],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut csv = WriterBuilder::new().from_writer(writer);

    let mut header = Vec::with_capacity(dataset.columns.len() + 3);
    if dataset.has_id {
        header.push("id".to_string());
    }
    for col in &dataset.columns {
        header.push(col.clone());
    }
    header.push("_cluster_id".to_string());
    header.push("_cluster_distance".to_string());
    csv.write_record(&header)?;

    for assigned in points {
        let mut row = Vec::with_capacity(header.len());
        if dataset.has_id {
            row.push(assigned.point.id.clone().unwrap_or_default());
        }
        for value in &assigned.point.coords {
            row.push(value.to_string());
        }
        row.push(assigned.cluster_id.to_string());
        row.push(assigned.cluster_distance.to_string());
        csv.write_record(&row)?;
    }
    csv.flush()?;
    Ok(())
}

pub fn write_neighbors<W: Write>(
    writer: W,
    rows: &[Neighbor],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut csv = WriterBuilder::new().from_writer(writer);
    csv.write_record(["centroid_id", "neighbor_id", "distance", "rank"])?;

    for row in rows {
        csv.write_record([
            row.centroid_id.to_string(),
            row.neighbor_id.to_string(),
            row.distance.to_string(),
            row.rank.to_string(),
        ])?;
    }
    csv.flush()?;
    Ok(())
}
