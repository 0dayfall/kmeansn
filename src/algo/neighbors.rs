use crate::algo::distance;
use crate::model::Centroid;

#[derive(Debug, Clone)]
pub struct Neighbor {
    pub centroid_id: usize,
    pub neighbor_id: usize,
    pub distance: f64,
    pub rank: usize,
}

pub fn centroid_neighbors(
    centroids: &[Centroid],
    limit: Option<usize>,
) -> Vec<Neighbor> {
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
