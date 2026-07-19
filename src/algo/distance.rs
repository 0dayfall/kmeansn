pub fn euclidean(a: &[f64], b: &[f64]) -> f64 {
    let mut sum = 0.0;
    for (x, y) in a.iter().zip(b.iter()) {
        let d = x - y;
        sum += d * d;
    }
    sum.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn euclidean_basics() {
        assert_eq!(euclidean(&[0.0, 0.0], &[3.0, 4.0]), 5.0);
        assert_eq!(euclidean(&[1.0], &[1.0]), 0.0);
        assert_eq!(euclidean(&[], &[]), 0.0);
    }

    #[test]
    fn euclidean_is_symmetric() {
        let a = [1.5, -2.0, 3.25];
        let b = [-0.5, 4.0, 1.0];
        assert_eq!(euclidean(&a, &b), euclidean(&b, &a));
    }
}
