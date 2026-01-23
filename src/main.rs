fn main() {
    if let Err(err) = kmeansn::cli::run() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}
