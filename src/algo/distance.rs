pub fn euclidean(a: &[f64], b: &[f64]) -> f64 {
    let mut sum = 0.0;
    for (x, y) in a.iter().zip(b.iter()) {
        let d = x - y;
        sum += d * d;
    }
    sum.sqrt()
}
