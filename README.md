# kmeansn

K-means clustering for CSV/NDJSON streams.

## Install

### Homebrew (macOS/Linux)

```
brew tap 0dayfall/tap
brew install 0dayfall/tap/kmeansn
```

To install from the latest `main`:

```
brew install --HEAD 0dayfall/tap/kmeansn
```

### Cargo (from source)

```
cargo install --path .
```

## Usage

Fit centroids:

```
kmeansn fit -k 3 --input data.csv --output centroids.json
```

Assign points:

```
kmeansn assign --centroids centroids.json --input data.csv > assigned.csv
```

Cluster neighbors:

```
kmeansn cluster-neighbors --centroids centroids.json --output neighbors.csv
```

Use `--input-format` and `--output-format` when reading from stdin or when the
extension is ambiguous. See `docs/schema.md` for full details.
