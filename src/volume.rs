//! Volume

/// Example
///
/// ```rust
/// let cube = bose::volume::cube(10.0);
/// assert_eq!(cube, 1000.0);
/// ```
pub fn cube(side: f64) -> f64 {
    return side * side * side;
}

/// Example
///
/// ```rust
/// let parallelepiped = bose::volume::parallelepiped(10.0, 20.0, 30.0);
/// assert_eq!(parallelepiped, 6000.0);
/// ```
pub fn parallelepiped(length: f64, width: f64, height: f64) -> f64 {
    return length * width * height;
}

/// Example
///
/// ```rust
/// let prism = bose::volume::prism(10.0, 20.0);
/// assert_eq!(prism, 200.0);
/// ```
pub fn prism(base: f64, height: f64) -> f64 {
    return base * height;
}

/// Example
///
/// ```rust
/// let cylinder = bose::volume::cylinder(10.0, 20.0);
/// assert_eq!(cylinder, 6283.185307179587);
/// ```
pub fn cylinder(radius: f64, height: f64) -> f64 {
    return std::f64::consts::PI * radius * radius * height;
}

/// Example
///
/// ```rust
/// let cone = bose::volume::cone(10.0, 20.0);
/// assert_eq!(cone, 66.66666666666667);
/// ```
pub fn cone(base: f64, height: f64) -> f64 {
    return (base * height) / 3.0;
}

/// Example
///
/// ```rust
/// let sphere = bose::volume::sphere(10.0);
/// assert_eq!(sphere, 4188.790204786391);
/// ```
pub fn sphere(radius: f64) -> f64 {
    return (4.0 / 3.0) * (std::f64::consts::PI * radius * radius * radius);
}
