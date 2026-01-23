use std::path::Path;

use clap::ValueEnum;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Format {
    Csv,
    Ndjson,
}

pub fn infer_format_from_path(path: &Path) -> Option<Format> {
    let ext = path.extension()?.to_string_lossy().to_ascii_lowercase();
    match ext.as_str() {
        "csv" => Some(Format::Csv),
        "ndjson" | "jsonl" => Some(Format::Ndjson),
        _ => None,
    }
}

pub fn resolve_input_format(
    input_path: Option<&Path>,
    explicit: Option<Format>,
) -> Result<Format, Box<dyn std::error::Error>> {
    if let Some(fmt) = explicit {
        return Ok(fmt);
    }
    if let Some(path) = input_path {
        if let Some(fmt) = infer_format_from_path(path) {
            return Ok(fmt);
        }
    }
    Err("unable to infer input format; use --input-format".into())
}

pub fn resolve_output_format(
    output_path: Option<&Path>,
    explicit: Option<Format>,
    fallback: Format,
) -> Result<Format, Box<dyn std::error::Error>> {
    if let Some(fmt) = explicit {
        return Ok(fmt);
    }
    if let Some(path) = output_path {
        if let Some(fmt) = infer_format_from_path(path) {
            return Ok(fmt);
        }
    }
    Ok(fallback)
}
