use crate::algo::kmeans;
use crate::cli::args::{FitArgs, InitArg};
use crate::io::{centroids_json, csv, format, ndjson, reader};
use crate::model::Dataset;

pub fn run(args: FitArgs) -> Result<(), Box<dyn std::error::Error>> {
    let input_source = reader::input_source(args.input.clone());
    let input_format = format::resolve_input_format(args.input.as_deref(), args.input_format)?;

    let mut dataset = Dataset::new();
    let mut input = reader::open_input(&input_source)?;
    let points = match input_format {
        crate::io::format::Format::Csv => csv::read_points(&mut input, &mut dataset)?,
        crate::io::format::Format::Ndjson => ndjson::read_points(&mut input, &mut dataset)?,
    };
    if dataset.columns.is_empty() {
        return Err("no feature columns found".into());
    }

    let init = match args.init {
        InitArg::KmeansPlusPlus => kmeans::Init::PlusPlus,
        InitArg::Random => kmeans::Init::Random,
    };
    let fit = kmeans::fit(&points, args.clusters, args.max_iters, args.seed, init)?;
    let centroids_file = centroids_json::CentroidsFile::from_fit(&dataset, &fit);

    let output_target = reader::output_target(args.output);
    let mut output = reader::open_output(&output_target)?;
    centroids_json::write_centroids(&mut output, &centroids_file)?;
    Ok(())
}
