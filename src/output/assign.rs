use crate::model::Point;

pub struct AssignedPoint {
    pub point: Point,
    pub cluster_id: usize,
    pub cluster_distance: f64,
}
