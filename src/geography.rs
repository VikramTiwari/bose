pub struct Location {
	pub latitude: f64,
	pub longitude: f64,
}

pub enum Unit {
	Mile,
	Kilometer,
}

pub fn to_radian(number: f64) -> f64 {
	return number.to_radians();
}

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
