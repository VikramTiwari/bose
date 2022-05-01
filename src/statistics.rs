//! Statistics

/// Example
///
/// ```rust
/// let average = bose::statistics::average(&[10.0, 20.0, 30.0, 40.0, 50.0]);
/// assert_eq!(average, 30.0);
/// ```
pub fn average(numbers: &[f64]) -> f64 {
    return numbers.iter().sum::<f64>() as f64 / numbers.len() as f64;
}

/// Example
///
/// ```rust
/// let mean = bose::statistics::mean(&[10.0, 20.0, 30.0, 40.0, 50.0]);
/// assert_eq!(mean, 30.0);
/// ```
pub fn mean(numbers: &[f64]) -> f64 {
    return numbers.iter().sum::<f64>() as f64 / numbers.len() as f64;
}
