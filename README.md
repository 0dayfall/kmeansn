# kmeansn

[![CI](https://github.com/0dayfall/kmeansn/actions/workflows/ci.yml/badge.svg)](https://github.com/0dayfall/kmeansn/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/kmeansn.svg)](https://crates.io/crates/kmeansn)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

K-means clustering for CSV/NDJSON streams. Fit, assign, and inspect
centroid neighborhoods straight from your shell -- no notebook required.

## Why kmeansn?

- **A filter, not a framework** -- reads stdin, writes stdout, composes with
  `jq`, `awk`, `sort`, and everything else in your pipeline. Clustering a CSV
  should not require spinning up Python and importing scikit-learn.
- **k-means++ initialization by default** -- better-spread starting centroids
  and more stable clusters than plain random seeding (use `--init random` to
  opt out).
- **Reproducible** -- pass `--seed` and two runs are byte-for-byte identical.
- **Versioned output schema** -- centroid files are tagged
  `kmeansn.centroids.v1`, so downstream scripts can rely on the format
  (see [docs/schema.md](docs/schema.md)).
- **Single static binary** -- no runtime, no dependencies.

```
$ kmeansn fit -k 2 --seed 42 --input data.csv | kmeansn assign --centroids /dev/stdin --input data.csv
id,x,y,_cluster_id,_cluster_distance
p1,1,2,0,0.047140452079
p2,1.2,2.1,0,0.213437474581
p3,0.9,1.8,0,0.213437474581
p4,8.5,9.1,1,0.235702260395
p5,9,9.2,1,0.298142396999
p6,8.7,8.9,1,0.235702260395
```

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

### Ubuntu (binary release)

```
VERSION=0.1.0
ARCH="$(uname -m)"
case "${ARCH}" in
  x86_64) ARCH=x86_64 ;;
  aarch64|arm64) ARCH=arm64 ;;
  *) echo "unsupported arch: ${ARCH}" && exit 1 ;;
esac

curl -L "https://github.com/0dayfall/kmeansn/releases/download/v${VERSION}/kmeansn_${VERSION}_Linux_${ARCH}.tar.gz" | tar -xz
sudo install -m755 kmeansn /usr/local/bin/kmeansn
```

### Cargo

```
cargo install kmeansn
```

Or from a checkout of this repository:

```
cargo install --path .
```

## Usage

### Quick start (CSV)

Create a small CSV:

```
cat <<'EOF' > data.csv
id,x,y
a,1.0,2.0
b,1.2,2.1
c,8.5,9.1
d,9.0,9.2
EOF
```

Fit centroids and assign:

```
kmeansn fit -k 2 --input data.csv --output centroids.json
kmeansn assign --centroids centroids.json --input data.csv > assigned.csv
```

### Quick start (NDJSON)

```
printf '%s\n' \
  '{"x":1.0,"y":2.0}' \
  '{"x":1.2,"y":2.1}' \
  '{"x":8.5,"y":9.1}' \
  '{"x":9.0,"y":9.2}' | \
  kmeansn fit -k 2 --input-format ndjson > centroids.json
```

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

### Options worth knowing

```
--seed <N>        reproducible runs
--init <STRATEGY> centroid initialization: kmeans++ (default) or random
--max-iters <N>   iteration cap (default 100)
--neighbors <N>   limit neighbor rows per centroid (cluster-neighbors)
```

## Development

```
cargo test          # unit + CLI integration tests
cargo clippy --all-targets -- -D warnings
cargo fmt --all --check
```

CI runs all three on every push.

## License

MIT -- see [LICENSE](LICENSE).
