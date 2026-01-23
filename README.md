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

### Cargo (from source)

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
