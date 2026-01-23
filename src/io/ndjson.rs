use std::collections::{BTreeMap, HashSet};
use std::io::{BufRead, Read, Write};

use serde_json::Value;

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
    let buf = std::io::BufReader::new(reader);
    let mut points = Vec::new();

    for line in buf.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let value: Value = serde_json::from_str(&line)?;
        let obj = value
            .as_object()
            .ok_or_else(|| "ndjson line must be an object".to_string())?;

        if dataset.columns.is_empty() {
            let mut cols: Vec<String> = obj
                .keys()
                .filter(|key| !is_reserved(key))
                .map(|key| key.to_string())
                .collect();
            cols.sort();
            dataset.columns = cols;
        } else {
            let actual: HashSet<&str> = obj
                .keys()
                .filter(|key| !is_reserved(key))
                .map(|key| key.as_str())
                .collect();
            let expected: HashSet<&str> = dataset.columns.iter().map(|s| s.as_str()).collect();
            if actual != expected {
                return Err("input fields do not match centroids columns".into());
            }
        }

        let id = obj.get("id").and_then(|v| v.as_str()).map(|s| s.to_string());
        if id.is_some() {
            dataset.has_id = true;
        }
        let mut coords = Vec::with_capacity(dataset.columns.len());
        for col in &dataset.columns {
            let raw = obj
                .get(col)
                .ok_or_else(|| "missing field".to_string())?;
            let value = raw
                .as_f64()
                .ok_or_else(|| "feature value must be a number".to_string())?;
            coords.push(value);
        }

        points.push(Point { id, coords });
    }

    Ok(points)
}

pub fn write_assigned_points<W: Write>(
    mut writer: W,
    dataset: &Dataset,
    points: &[AssignedPoint],
) -> Result<(), Box<dyn std::error::Error>> {
    for assigned in points {
        let mut obj: BTreeMap<String, Value> = BTreeMap::new();
        if dataset.has_id {
            if let Some(id) = &assigned.point.id {
                obj.insert("id".to_string(), Value::String(id.clone()));
            }
        }
        for (col, value) in dataset.columns.iter().zip(assigned.point.coords.iter()) {
            obj.insert(col.clone(), Value::Number(serde_json::Number::from_f64(*value).ok_or_else(|| "invalid float".to_string())?));
        }
        obj.insert(
            "_cluster_id".to_string(),
            Value::Number(serde_json::Number::from(assigned.cluster_id as i64)),
        );
        obj.insert(
            "_cluster_distance".to_string(),
            Value::Number(serde_json::Number::from_f64(assigned.cluster_distance)
                .ok_or_else(|| "invalid float".to_string())?),
        );

        let line = serde_json::to_string(&obj)?;
        writeln!(writer, "{line}")?;
    }
    Ok(())
}

pub fn write_neighbors<W: Write>(
    mut writer: W,
    rows: &[Neighbor],
) -> Result<(), Box<dyn std::error::Error>> {
    for row in rows {
        let mut obj = BTreeMap::new();
        obj.insert(
            "centroid_id".to_string(),
            Value::Number(serde_json::Number::from(row.centroid_id as i64)),
        );
        obj.insert(
            "neighbor_id".to_string(),
            Value::Number(serde_json::Number::from(row.neighbor_id as i64)),
        );
        obj.insert(
            "distance".to_string(),
            Value::Number(serde_json::Number::from_f64(row.distance)
                .ok_or_else(|| "invalid float".to_string())?),
        );
        obj.insert(
            "rank".to_string(),
            Value::Number(serde_json::Number::from(row.rank as i64)),
        );
        let line = serde_json::to_string(&obj)?;
        writeln!(writer, "{line}")?;
    }
    Ok(())
}
