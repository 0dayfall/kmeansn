use rand::rngs::StdRng;
use rand::seq::index::sample;
use rand::{Rng, SeedableRng};

use crate::algo::distance;
use crate::model::{Centroid, Point};

/// Centroid initialization strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Init {
    /// k-means++ seeding: spread initial centroids apart for better,
    /// more stable clusters (Arthur & Vassilvitskii, 2007).
    #[default]
    PlusPlus,
    /// Uniform random sampling of k distinct input points.
    Random,
}

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
    init: Init,
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
        None => StdRng::from_os_rng(),
    };

    let mut centroids = match init {
        Init::PlusPlus => init_plus_plus(points, k, &mut rng),
        Init::Random => init_random(points, k, &mut rng),
    };

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
            for (sum, coord) in sums[best].iter_mut().zip(point.coords.iter()) {
                *sum += coord;
            }
            sse[best] += best_dist * best_dist;
        }

        let mut new_centroids = Vec::with_capacity(k);
        for ci in 0..k {
            let coords = if counts[ci] == 0 {
                let idx = rng.random_range(0..points.len());
                points[idx].coords.clone()
            } else {
                sums[ci].iter().map(|v| v / counts[ci] as f64).collect()
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

/// Uniform random selection of k distinct points as initial centroids.
fn init_random(points: &[Point], k: usize, rng: &mut StdRng) -> Vec<Centroid> {
    sample(rng, points.len(), k)
        .iter()
        .enumerate()
        .map(|(id, idx)| Centroid {
            id,
            coords: points[idx].coords.clone(),
            size: None,
            sse: None,
        })
        .collect()
}

/// k-means++ seeding: the first centroid is drawn uniformly; each subsequent
/// centroid is drawn with probability proportional to the squared distance
/// from the nearest centroid chosen so far.
fn init_plus_plus(points: &[Point], k: usize, rng: &mut StdRng) -> Vec<Centroid> {
    let mut chosen: Vec<usize> = Vec::with_capacity(k);
    chosen.push(rng.random_range(0..points.len()));

    // Squared distance from each point to its nearest chosen centroid.
    let mut min_sq_dist: Vec<f64> = points
        .iter()
        .map(|p| {
            let d = distance::euclidean(&p.coords, &points[chosen[0]].coords);
            d * d
        })
        .collect();

    while chosen.len() < k {
        let total: f64 = min_sq_dist.iter().sum();
        let next = if total > 0.0 {
            // Weighted draw proportional to squared distance.
            let mut target = rng.random_range(0.0..total);
            let mut pick = 0;
            for (idx, w) in min_sq_dist.iter().enumerate() {
                if target < *w {
                    pick = idx;
                    break;
                }
                target -= w;
                pick = idx;
            }
            pick
        } else {
            // All remaining points coincide with chosen centroids;
            // fall back to a uniform draw.
            rng.random_range(0..points.len())
        };

        chosen.push(next);
        for (idx, point) in points.iter().enumerate() {
            let d = distance::euclidean(&point.coords, &points[next].coords);
            let sq = d * d;
            if sq < min_sq_dist[idx] {
                min_sq_dist[idx] = sq;
            }
        }
    }

    chosen
        .into_iter()
        .enumerate()
        .map(|(id, idx)| Centroid {
            id,
            coords: points[idx].coords.clone(),
            size: None,
            sse: None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn point(coords: &[f64]) -> Point {
        Point {
            id: None,
            coords: coords.to_vec(),
        }
    }

    fn two_cluster_data() -> Vec<Point> {
        vec![
            point(&[1.0, 1.0]),
            point(&[1.1, 0.9]),
            point(&[0.9, 1.1]),
            point(&[10.0, 10.0]),
            point(&[10.1, 9.9]),
            point(&[9.9, 10.1]),
        ]
    }

    #[test]
    fn finds_two_well_separated_clusters() {
        let points = two_cluster_data();
        let result = fit(&points, 2, 100, Some(7), Init::PlusPlus).unwrap();

        assert!(result.converged);
        assert_eq!(result.data_count, 6);
        assert_eq!(result.centroids.len(), 2);

        let mut means: Vec<Vec<f64>> = result.centroids.iter().map(|c| c.coords.clone()).collect();
        means.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());

        assert!((means[0][0] - 1.0).abs() < 0.2, "low centroid x: {means:?}");
        assert!((means[0][1] - 1.0).abs() < 0.2, "low centroid y: {means:?}");
        assert!(
            (means[1][0] - 10.0).abs() < 0.2,
            "high centroid x: {means:?}"
        );
        assert!(
            (means[1][1] - 10.0).abs() < 0.2,
            "high centroid y: {means:?}"
        );

        let sizes: usize = result.centroids.iter().map(|c| c.size.unwrap()).sum();
        assert_eq!(sizes, 6);
    }

    #[test]
    fn seeded_runs_are_deterministic() {
        let points = two_cluster_data();
        for init in [Init::PlusPlus, Init::Random] {
            let a = fit(&points, 2, 100, Some(42), init).unwrap();
            let b = fit(&points, 2, 100, Some(42), init).unwrap();
            for (ca, cb) in a.centroids.iter().zip(b.centroids.iter()) {
                assert_eq!(ca.coords, cb.coords);
            }
            assert_eq!(a.iterations, b.iterations);
        }
    }

    #[test]
    fn k_equal_to_point_count_is_allowed() {
        let points = two_cluster_data();
        let result = fit(&points, 6, 10, Some(1), Init::PlusPlus).unwrap();
        assert_eq!(result.centroids.len(), 6);
    }

    #[test]
    fn rejects_empty_input() {
        let err = fit(&[], 2, 100, None, Init::PlusPlus).unwrap_err();
        assert!(err.to_string().contains("no input points"));
    }

    #[test]
    fn rejects_zero_k() {
        let points = two_cluster_data();
        let err = fit(&points, 0, 100, None, Init::PlusPlus).unwrap_err();
        assert!(err.to_string().contains("k must be greater than 0"));
    }

    #[test]
    fn rejects_k_larger_than_input() {
        let points = two_cluster_data();
        let err = fit(&points, 7, 100, None, Init::PlusPlus).unwrap_err();
        assert!(err.to_string().contains("k must be <="));
    }

    #[test]
    fn rejects_zero_max_iters() {
        let points = two_cluster_data();
        let err = fit(&points, 2, 0, None, Init::PlusPlus).unwrap_err();
        assert!(err.to_string().contains("max_iters"));
    }

    #[test]
    fn rejects_inconsistent_dimensions() {
        let points = vec![point(&[1.0, 2.0]), point(&[1.0])];
        let err = fit(&points, 1, 100, None, Init::PlusPlus).unwrap_err();
        assert!(err.to_string().contains("dimensionality"));
    }

    #[test]
    fn rejects_zero_dimensional_points() {
        let points = vec![point(&[]), point(&[])];
        let err = fit(&points, 1, 100, None, Init::PlusPlus).unwrap_err();
        assert!(err.to_string().contains("dimension"));
    }

    #[test]
    fn handles_duplicate_points() {
        // All points identical: k-means++ falls back to uniform draws.
        let points = vec![point(&[5.0, 5.0]); 4];
        let result = fit(&points, 2, 10, Some(3), Init::PlusPlus).unwrap();
        assert_eq!(result.centroids.len(), 2);
        for c in &result.centroids {
            assert_eq!(c.coords, vec![5.0, 5.0]);
        }
    }

    #[test]
    fn plus_plus_spreads_initial_centroids() {
        // With two tight groups far apart, k-means++ should almost always
        // seed one centroid in each group; verify via a converged fit.
        let points = two_cluster_data();
        let result = fit(&points, 2, 100, Some(0), Init::PlusPlus).unwrap();
        let sizes: Vec<usize> = result.centroids.iter().map(|c| c.size.unwrap()).collect();
        assert_eq!(sizes, vec![3, 3]);
    }
}
