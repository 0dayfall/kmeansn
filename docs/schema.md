# kmeansn data schema

This document defines the input/output schemas for CSV and NDJSON.

## General rules

- One row/object = one point.
- Optional `id` (string).
- Any field starting with `_` is reserved for tool output and ignored on input.
- All other fields must be numeric feature values (float).
- Feature column order is defined by the input header (CSV) or by the centroids file. For NDJSON `fit` without a centroids file, feature order is the sorted key order of the first object.

## fit input (CSV)

- Header required.
- All non-reserved columns must be numeric.

Example:
```
id,x,y,z
p1,1.2,3.4,5.6
p2,2.0,1.0,0.5
```

## fit input (NDJSON)

- One JSON object per line.
- All non-reserved fields must be numeric.

Example:
```
{"id":"p1","x":1.2,"y":3.4,"z":5.6}
{"id":"p2","x":2.0,"y":1.0,"z":0.5}
```

## fit output (centroids JSON)

Single JSON document written to stdout (or file). This is the explicit state for later commands.

Schema:
```
{
  "version": "kmeansn.centroids.v1",
  "dims": 3,
  "columns": ["x","y","z"],
  "distance": "euclidean",
  "centroids": [
    {
      "id": 0,
      "coords": [1.5, 2.2, 0.7],
      "size": 42,
      "sse": 12.345
    }
  ],
  "data_count": 123,
  "converged": true,
  "iterations": 12
}
```

Notes:
- `columns` defines the canonical feature order.
- `id` is stable cluster id (0..k-1).
- `size` and `sse` are optional but recommended.
- `distance` is an enum string; start with `euclidean`.

## assign input

Same as `fit` input (CSV/NDJSON). Requires `--centroids` pointing to the centroids JSON.

## assign output

Same as input with fields appended. If `id` was present in the input, it is preserved:
- `_cluster_id` (int)
- `_cluster_distance` (float)

CSV example:
```
id,x,y,z,_cluster_id,_cluster_distance
p1,1.2,3.4,5.6,2,0.134
```

NDJSON example:
```
{"id":"p1","x":1.2,"y":3.4,"z":5.6,"_cluster_id":2,"_cluster_distance":0.134}
```

## cluster-neighbors input

- Centroids JSON (same as `fit` output). No point stream required.

## cluster-neighbors output

Streamable pairs of centroid distances.

CSV columns:
- `centroid_id,neighbor_id,distance,rank`

NDJSON fields:
- `centroid_id`, `neighbor_id`, `distance`, `rank`

CSV example:
```
centroid_id,neighbor_id,distance,rank
0,2,0.431,1
0,1,0.982,2
```

NDJSON example:
```
{"centroid_id":0,"neighbor_id":2,"distance":0.431,"rank":1}
{"centroid_id":0,"neighbor_id":1,"distance":0.982,"rank":2}
```
