use crate::algo::distance;
use crate::model::Centroid;

#[derive(Debug, Clone)]
pub struct Neighbor {
    pub centroid_id: usize,
    pub neighbor_id: usize,
    pub distance: f64,
    pub rank: usize,
}

pub fn centroid_neighbors(centroids: &[Centroid], limit: Option<usize>) -> Vec<Neighbor> {
    let mut rows = Vec::new();

    for (i, centroid) in centroids.iter().enumerate() {
        let mut distances = Vec::with_capacity(centroids.len().saturating_sub(1));
        for (j, other) in centroids.iter().enumerate() {
            if i == j {
                continue;
            }
            let dist = distance::euclidean(&centroid.coords, &other.coords);
            distances.push((j, dist));
        }

        distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        let take = limit.unwrap_or(distances.len());
        for (rank, (j, dist)) in distances.into_iter().take(take).enumerate() {
            rows.push(Neighbor {
                centroid_id: centroid.id,
                neighbor_id: centroids[j].id,
                distance: dist,
                rank: rank + 1,
            });
        }
    }

    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    fn centroid(id: usize, coords: &[f64]) -> Centroid {
        Centroid {
            id,
            coords: coords.to_vec(),
            size: None,
            sse: None,
        }
    }

    #[test]
    fn ranks_neighbors_by_distance() {
        let centroids = vec![
            centroid(0, &[0.0, 0.0]),
            centroid(1, &[1.0, 0.0]),
            centroid(2, &[5.0, 0.0]),
        ];
        let rows = centroid_neighbors(&centroids, None);
        assert_eq!(rows.len(), 6); // 3 centroids x 2 neighbors each

        let for_zero: Vec<_> = rows.iter().filter(|r| r.centroid_id == 0).collect();
        assert_eq!(for_zero[0].neighbor_id, 1);
        assert_eq!(for_zero[0].rank, 1);
        assert_eq!(for_zero[0].distance, 1.0);
        assert_eq!(for_zero[1].neighbor_id, 2);
        assert_eq!(for_zero[1].rank, 2);
        assert_eq!(for_zero[1].distance, 5.0);
    }

    #[test]
    fn respects_neighbor_limit() {
        let centroids = vec![
            centroid(0, &[0.0]),
            centroid(1, &[1.0]),
            centroid(2, &[2.0]),
        ];
        let rows = centroid_neighbors(&centroids, Some(1));
        assert_eq!(rows.len(), 3); // one neighbor per centroid
        assert!(rows.iter().all(|r| r.rank == 1));
    }

    #[test]
    fn single_centroid_has_no_neighbors() {
        let centroids = vec![centroid(0, &[1.0, 2.0])];
        assert!(centroid_neighbors(&centroids, None).is_empty());
    }
}
