pub fn cnt_increasing_measurements(measurements: &[u32]) -> usize {
    measurements
        .iter()
        .zip(measurements[1..].iter())
        .filter(|(m1, m2)| m1 < m2)
        .count()
}
