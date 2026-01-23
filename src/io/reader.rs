use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum InputSource {
    Stdin,
    File(PathBuf),
}

#[derive(Debug, Clone)]
pub enum OutputTarget {
    Stdout,
    File(PathBuf),
}

pub fn input_source(path: Option<PathBuf>) -> InputSource {
    match path {
        Some(path) => InputSource::File(path),
        None => InputSource::Stdin,
    }
}

pub fn output_target(path: Option<PathBuf>) -> OutputTarget {
    match path {
        Some(path) => OutputTarget::File(path),
        None => OutputTarget::Stdout,
    }
}

pub fn open_input(source: &InputSource) -> Result<Box<dyn BufRead>, Box<dyn std::error::Error>> {
    match source {
        InputSource::Stdin => Ok(Box::new(BufReader::new(std::io::stdin()))),
        InputSource::File(path) => Ok(Box::new(BufReader::new(File::open(path)?))),
    }
}

pub fn open_output(
    target: &OutputTarget,
) -> Result<Box<dyn Write>, Box<dyn std::error::Error>> {
    match target {
        OutputTarget::Stdout => Ok(Box::new(BufWriter::new(std::io::stdout()))),
        OutputTarget::File(path) => Ok(Box::new(BufWriter::new(File::create(path)?))),
    }
}
