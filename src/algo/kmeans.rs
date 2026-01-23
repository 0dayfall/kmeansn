use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rand::seq::index::sample;

use crate::algo::distance;
use crate::model::{Centroid, Point};

#[derive(Debug, Clone)]
pub struct FitResult {
    pub centroids: Vec<Centroid>,
    pub data_count: usize,
    pub converged: bool,
    pub iterations: usize,
}

pub fn fit(
    points: &[Point],
    k: usize,
    max_iters: usize,
    seed: Option<u64>,
) -> Result<FitResult, Box<dyn std::error::Error>> {
    if points.is_empty() {
        return Err("no input points".into());
    }
    if k == 0 {
        return Err("k must be greater than 0".into());
    }
    if k > points.len() {
        return Err("k must be <= number of points".into());
    }
    if max_iters == 0 {
        return Err("max_iters must be greater than 0".into());
    }
    let dims = points[0].coords.len();
    if dims == 0 {
        return Err("points must have at least one dimension".into());
    }
    for point in points {
        if point.coords.len() != dims {
            return Err("inconsistent point dimensionality".into());
        }
    }

    let mut rng = match seed {
        Some(seed) => StdRng::seed_from_u64(seed),
        None => StdRng::from_entropy(),
    };

    let sample_indices = sample(&mut rng, points.len(), k);
    let mut centroids: Vec<Centroid> = sample_indices
        .iter()
        .enumerate()
        .map(|(id, idx)| Centroid {
            id,
            coords: points[idx].coords.clone(),
            size: None,
            sse: None,
        })
        .collect();

    let mut assignments = vec![usize::MAX; points.len()];
    let mut converged = false;
    let mut iterations = 0;

    for iter in 0..max_iters {
        let mut sums = vec![vec![0.0; dims]; k];
        let mut counts = vec![0usize; k];
        let mut sse = vec![0.0; k];
        let mut changed = false;

        for (pi, point) in points.iter().enumerate() {
            let mut best = 0;
            let mut best_dist = f64::INFINITY;
            for (ci, centroid) in centroids.iter().enumerate() {
                let dist = distance::euclidean(&point.coords, &centroid.coords);
                if dist < best_dist {
                    best_dist = dist;
                    best = ci;
                }
            }

            if assignments[pi] != best {
                assignments[pi] = best;
                changed = true;
            }

            counts[best] += 1;
            for d in 0..dims {
                sums[best][d] += point.coords[d];
            }
            sse[best] += best_dist * best_dist;
        }

        let mut new_centroids = Vec::with_capacity(k);
        for ci in 0..k {
            let coords = if counts[ci] == 0 {
                let idx = rng.gen_range(0..points.len());
                points[idx].coords.clone()
            } else {
                sums[ci]
                    .iter()
                    .map(|v| v / counts[ci] as f64)
                    .collect()
            };
            new_centroids.push(Centroid {
                id: ci,
                coords,
                size: Some(counts[ci]),
                sse: Some(sse[ci]),
            });
        }

        let mut max_shift = 0.0;
        for (old, new) in centroids.iter().zip(new_centroids.iter()) {
            let shift = distance::euclidean(&old.coords, &new.coords);
            if shift > max_shift {
                max_shift = shift;
            }
        }

        centroids = new_centroids;
        iterations = iter + 1;
        if !changed || max_shift < 1e-6 {
            converged = true;
            break;
        }
    }

    Ok(FitResult {
        centroids,
        data_count: points.len(),
        converged,
        iterations,
    })
}
