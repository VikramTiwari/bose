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
pub const c: f64 = 299792458.0; // m/s

/// speed of sound in air at 25 deg C
pub const c_in_air: f64 = 343.0; // m/s

/// speed of sound in iron at 25 deg C
pub const c_in_iron: f64 = 5120.0; // m/s

/// Plank's constant: J Hz^-1
pub const h: f64 = 6.62607015e-34;

/// Avogadro's constant: mol^-1
pub const avogadro: f64 = 6.02214076e23;

/// Boltzman's constant: J K^-1
pub const boltzman: f64 = 1.380649e-23;

/// Multiples for base unit

/// prefix: tera || symbol: T || meaning: trillion
pub const tera: f64 = 1e12;

/// prefix: giga || symbol: G || meaning: billion
pub const giga: f64 = 1e9;

/// prefix: mega || symbol: M || meaning: million
pub const mega: f64 = 1e6;

/// prefix: kilo || symbol: k || meaning: thousand
pub const kilo: f64 = 1e3;

/// prefix: centi || symbol: c || meaning: one hundredth
pub const centi: f64 = 1e-2;

/// prefix: milli || symbol: m || meaning: one thousandth
pub const milli: f64 = 1e-3;

/// prefix: micro || symbol: u || meaning: one millonth
pub const micro: f64 = 1e-6;

/// prefix: nano || symbol: n || meaning: one billionth
pub const nano: f64 = 1e-9;

/// prefix: pico || symbol: p || meaning: one trillion
pub const pico: f64 = 1e-12;
