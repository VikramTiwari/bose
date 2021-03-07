//! Perimeter

/// Example
/// ```rust
/// let circle = bose::perimeter::circle(10.0);
/// assert_eq!(62.83185307179586, circle);
/// ```
pub fn circle(radius: f64) -> f64 {
	return 2.0 * std::f64::consts::PI * radius;
}
