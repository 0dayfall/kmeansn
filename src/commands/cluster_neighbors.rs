use crate::algo::neighbors;
use crate::cli::args::ClusterNeighborsArgs;
use crate::io::format::Format;
use crate::io::{centroids_json, csv, format, ndjson, reader};

pub fn run(args: ClusterNeighborsArgs) -> Result<(), Box<dyn std::error::Error>> {
    let mut centroids_reader = reader::open_input(&reader::InputSource::File(args.centroids))?;
    let centroids_file = centroids_json::read_centroids(&mut centroids_reader)?;

    let output_format = format::resolve_output_format(
        args.output.as_deref(),
        args.output_format,
        Format::Csv,
    )?;

    let rows = neighbors::centroid_neighbors(&centroids_file.centroids, args.neighbors);

    let output_target = reader::output_target(args.output);
    let mut output = reader::open_output(&output_target)?;
    match output_format {
        Format::Csv => csv::write_neighbors(&mut output, &rows)?,
        Format::Ndjson => ndjson::write_neighbors(&mut output, &rows)?,
    }
    Ok(())
}
