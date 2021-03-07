//! Geography

/// Location struct
pub struct Location {
	/// latitude of the location
	pub latitude: f64,
	/// longitude of the location
	pub longitude: f64,
}

/// Enum for unit of distance
pub enum Unit {
	/// miles
	Mile,
	/// kilometers
	Kilometer,
}

/// Example
/// ```rust
/// let radian = bose::geography::to_radian(10.0);
/// assert_eq!(0.17453292519943295, radian);
/// ```
pub fn to_radian(number: f64) -> f64 {
	return number.to_radians();
}

/// Example
/// ```rust
/// let haversine_distance_in_miles = bose::geography::distance(
/// 	bose::geography::Location {
/// 		latitude: 48.85341,
/// 		longitude: -2.34880,
/// 	},
/// 	bose::geography::Location {
/// 		latitude: 51.50853,
/// 		longitude: -0.12574,
/// 	},
/// 	bose::geography::Unit::Mile,
/// );
/// assert_eq!(194.5908231787244, haversine_distance_in_miles);
/// let haversine_distance_in_kilometers = bose::geography::distance(
/// 	bose::geography::Location {
/// 		latitude: 48.85341,
/// 		longitude: -2.34880,
/// 	},
/// 	bose::geography::Location {
/// 		latitude: 51.50853,
/// 		longitude: -0.12574,
/// 	},
/// 	bose::geography::Unit::Kilometer,
/// );
/// assert_eq!(313.0651854726397, haversine_distance_in_kilometers);
/// ```
pub fn distance(start: Location, end: Location, unit: Unit) -> f64 {
	let earth_radius_in_kilometers: f64 = 6371.0;
	let earth_radius_in_miles: f64 = 3960.0;
	let mut earth_radius: f64 = 0.0;

	match unit {
		Unit::Mile => earth_radius = earth_radius_in_miles,
		Unit::Kilometer => earth_radius = earth_radius_in_kilometers,
	}

	let start_latitude_in_radians: f64 = (start.latitude).to_radians();
	let end_latitude_in_radians: f64 = (end.latitude).to_radians();

	let delta_latitude: f64 = (end.latitude - start.latitude).to_radians();
	let delta_longitude: f64 = (end.longitude - start.longitude).to_radians();

	let central_angle_inner: f64 = ((delta_latitude / 2.0).sin()) * ((delta_longitude / 2.0).sin())
		+ ((delta_longitude / 2.0).sin())
			* ((delta_longitude / 2.0).sin())
			* (start_latitude_in_radians.cos())
			* (end_latitude_in_radians.cos());

	let central_angle: f64 =
		2.0 * ((central_angle_inner.sqrt()).atan2((1.0 - central_angle_inner).sqrt()));

	return earth_radius * central_angle;
}
