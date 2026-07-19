use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

use crate::io::format::Format;

#[derive(Parser, Debug)]
#[command(name = "kmeansn")]
#[command(version)]
#[command(about = "K-means clustering for CSV/NDJSON streams", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Fit(FitArgs),
    Assign(AssignArgs),
    ClusterNeighbors(ClusterNeighborsArgs),
}

#[derive(Args, Debug)]
pub struct FitArgs {
    /// Input file (defaults to stdin)
    #[arg(short, long)]
    pub input: Option<PathBuf>,

    /// Output centroids JSON file (defaults to stdout)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Input format when reading from stdin or ambiguous extension
    #[arg(long)]
    pub input_format: Option<Format>,

    /// Number of clusters
    #[arg(short = 'k', long)]
    pub clusters: usize,

    /// Maximum iterations
    #[arg(long, default_value_t = 100)]
    pub max_iters: usize,

    /// RNG seed for centroid initialization
    #[arg(long)]
    pub seed: Option<u64>,

    /// Centroid initialization strategy
    #[arg(long, value_enum, default_value_t = InitArg::KmeansPlusPlus)]
    pub init: InitArg,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum InitArg {
    /// k-means++ seeding (better spread, more stable clusters)
    #[value(name = "kmeans++", alias = "plusplus")]
    KmeansPlusPlus,
    /// Uniform random sampling of input points
    Random,
}

#[derive(Args, Debug)]
pub struct AssignArgs {
    /// Input file (defaults to stdin)
    #[arg(short, long)]
    pub input: Option<PathBuf>,

    /// Output file (defaults to stdout)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Input format when reading from stdin or ambiguous extension
    #[arg(long)]
    pub input_format: Option<Format>,

    /// Output format override
    #[arg(long)]
    pub output_format: Option<Format>,

    /// Centroids JSON file
    #[arg(long)]
    pub centroids: PathBuf,
}

#[derive(Args, Debug)]
pub struct ClusterNeighborsArgs {
    /// Output file (defaults to stdout)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Output format override
    #[arg(long)]
    pub output_format: Option<Format>,

    /// Centroids JSON file
    #[arg(long)]
    pub centroids: PathBuf,

    /// Limit number of neighbors per centroid
    #[arg(long)]
    pub neighbors: Option<usize>,
}
