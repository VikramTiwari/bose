//! Volume

/// Example
/// ```rust
/// let cube = bose::volume::cube(10.0);
/// assert_eq!(1000.0, cube);
/// ```
pub fn cube(side: f64) -> f64 {
	return side * side * side;
}

/// Example
/// ```rust
/// let average = bose::statistics::average(&[10.0, 20.0, 30.0, 40.0, 50.0]);
/// assert_eq!(30.0, average);
/// ```
pub fn parallelepiped(length: f64, width: f64, height: f64) -> f64 {
	return length * width * height;
}

/// Example
/// ```rust
/// let average = bose::statistics::average(&[10.0, 20.0, 30.0, 40.0, 50.0]);
/// assert_eq!(30.0, average);
/// ```
pub fn prism(base: f64, height: f64) -> f64 {
	return base * height;
}

/// Example
/// ```rust
/// let average = bose::statistics::average(&[10.0, 20.0, 30.0, 40.0, 50.0]);
/// assert_eq!(30.0, average);
/// ```
pub fn cylinder(radius: f64, height: f64) -> f64 {
	return std::f64::consts::PI * radius * radius * height;
}

/// Example
/// ```rust
/// let average = bose::statistics::average(&[10.0, 20.0, 30.0, 40.0, 50.0]);
/// assert_eq!(30.0, average);
/// ```
pub fn cone(base: f64, height: f64) -> f64 {
	return (base * height) / 3.0;
}

/// Example
/// ```rust
/// let average = bose::statistics::average(&[10.0, 20.0, 30.0, 40.0, 50.0]);
/// assert_eq!(30.0, average);
/// ```
pub fn sphere(radius: f64) -> f64 {
	return (4.0 / 3.0) * (std::f64::consts::PI * radius * radius * radius);
}
