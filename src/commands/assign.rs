use crate::algo::distance;
use crate::cli::args::AssignArgs;
use crate::io::format::Format;
use crate::io::{centroids_json, csv, format, ndjson, reader};
use crate::model::{Dataset, Point};
use crate::output::assign::AssignedPoint;

pub fn run(args: AssignArgs) -> Result<(), Box<dyn std::error::Error>> {
    let mut centroids_reader = reader::open_input(&reader::InputSource::File(args.centroids))?;
    let centroids_file = centroids_json::read_centroids(&mut centroids_reader)?;
    if centroids_file.centroids.is_empty() {
        return Err("centroids file contains no centroids".into());
    }

    let mut dataset = Dataset {
        columns: centroids_file.columns.clone(),
        has_id: false,
    };

    let input_source = reader::input_source(args.input.clone());
    let input_format = format::resolve_input_format(args.input.as_deref(), args.input_format)?;
    let output_format = format::resolve_output_format(
        args.output.as_deref(),
        args.output_format,
        input_format,
    )?;

    let mut input = reader::open_input(&input_source)?;
    let points = match input_format {
        Format::Csv => csv::read_points(&mut input, &mut dataset)?,
        Format::Ndjson => ndjson::read_points(&mut input, &mut dataset)?,
    };

    let assigned = assign_points(&points, &centroids_file.centroids);

    let output_target = reader::output_target(args.output);
    let mut output = reader::open_output(&output_target)?;
    match output_format {
        Format::Csv => csv::write_assigned_points(&mut output, &dataset, &assigned)?,
        Format::Ndjson => ndjson::write_assigned_points(&mut output, &dataset, &assigned)?,
    }
    Ok(())
}

fn assign_points(points: &[Point], centroids: &[crate::model::Centroid]) -> Vec<AssignedPoint> {
    let mut assigned = Vec::with_capacity(points.len());
    for point in points {
        let mut best_id = centroids.first().map(|c| c.id).unwrap_or(0);
        let mut best_dist = f64::INFINITY;
        for centroid in centroids {
            let dist = distance::euclidean(&point.coords, &centroid.coords);
            if dist < best_dist {
                best_id = centroid.id;
                best_dist = dist;
            }
        }
        assigned.push(AssignedPoint {
            point: point.clone(),
            cluster_id: best_id,
            cluster_distance: best_dist,
        });
    }
    assigned
}
