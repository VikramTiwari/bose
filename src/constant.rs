#![allow(non_upper_case_globals)]
//! Constants

// various scientific constants from various locations
// https://physics.nist.gov/cuu/Constants/Table/allascii.txt

/// Speed of light in vaccum measured: m s^-1
///
/// Example
/// ```rust
/// let distance_light_travels_2_seconds = bose::constant::c * 2.0;
/// assert_eq!(599584916.0, distance_light_travels_2_seconds);
/// ```
pub const c: f64 = 299792458.0;

/// Plank's constant: J Hz^-1
pub const h: f64 = 6.62607015e-34;

/// Avogadro's constant: mol^-1
pub const avogadro: f64 = 6.02214076e23;

/// Boltzman's constant: J K^-1
pub const boltzman: f64 = 1.380649e-23;
