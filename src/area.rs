//! Area

/// Example
///
/// ```rust
/// let square = bose::area::square(10.0);
/// assert_eq!(square, 100.0);
/// ```
pub fn square(length: f64) -> f64 {
    return length * length;
}

/// Example
///
/// ```rust
/// let rectangle = bose::area::rectangle(10.0, 5.0);
/// assert_eq!(rectangle, 50.0);
/// ```
pub fn rectangle(length: f64, width: f64) -> f64 {
    return length * width;
}

/// Example
///
/// ```rust
/// let triangle = bose::area::triangle(10.0, 5.0);
/// assert_eq!(triangle, 25.0);
/// ```
pub fn triangle(base: f64, height: f64) -> f64 {
    return (base * height) / 2.0;
}

/// Example
///
/// ```rust
/// let rhombus = bose::area::rhombus(10.0, 10.0);
/// assert_eq!(rhombus, 50.0);
/// ```
pub fn rhombus(large_diagonal: f64, small_diagonal: f64) -> f64 {
    return (large_diagonal * small_diagonal) / 2.0;
}

/// Example
///
/// ```rust
/// let trapezoid = bose::area::trapezoid(10.0, 5.0, 2.0);
/// assert_eq!(trapezoid, 15.0);
/// ```
pub fn trapezoid(large_side: f64, small_side: f64, height: f64) -> f64 {
    return ((large_side + small_side) / 2.0) * height;
}

/// Example
///
/// ```rust
/// let polygon = bose::area::polygon(10.0, 5.0);
/// assert_eq!(polygon, 25.0);
/// ```
pub fn polygon(perimeter: f64, apothem: f64) -> f64 {
    return (perimeter / 2.0) * apothem;
}

/// Example
///
/// ```rust
/// let circle = bose::area::circle(10.0);
/// assert_eq!(circle, 314.1592653589793);
/// ```
pub fn circle(radius: f64) -> f64 {
    return std::f64::consts::PI * radius * radius;
}

/// Example
///
/// ```rust
/// let cone = bose::area::cone(10.0, 5.0);
/// assert_eq!(cone, 157.07963267948966);
/// ```
pub fn cone(radius: f64, slant_height: f64) -> f64 {
    return std::f64::consts::PI * radius * slant_height;
}

/// Example
///
/// ```rust
/// let sphere = bose::area::sphere(10.0);
/// assert_eq!(sphere, 1256.6370614359173);
/// ```
pub fn sphere(radius: f64) -> f64 {
    return 4.0 * std::f64::consts::PI * radius * radius;
}
