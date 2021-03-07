pub mod area {
	pub fn square(length: f64) -> f64 {
		return length * length;
	}
	pub fn rectangle(length: f64, width: f64) -> f64 {
		return length * width;
	}
	pub fn triangle(base: f64, height: f64) -> f64 {
		return (base * height) / 2.0;
	}
	pub fn rhombus(large_diagonal: f64, small_diagonal: f64) -> f64 {
		return (large_diagonal * small_diagonal) / 2.0;
	}
	pub fn trapezoid(large_side: f64, small_side: f64, height: f64) -> f64 {
		return ((large_side + small_side) / 2.0) * height;
	}
	pub fn polygon(perimeter: f64, apothem: f64) -> f64 {
		return (perimeter / 2.0) * apothem;
	}
	pub fn circle(radius: f64) -> f64 {
		return std::f64::consts::PI * radius * radius;
	}
	pub fn cone(radius: f64, slant_height: f64) -> f64 {
		return std::f64::consts::PI * radius * slant_height;
	}
	pub fn sphere(radius: f64) -> f64 {
		return 4.0 * std::f64::consts::PI * radius * radius;
	}
}

pub mod perimeter {
	pub fn circle(radius: f64) -> f64 {
		return 2.0 * std::f64::consts::PI * radius;
	}
}

pub mod volume {
	pub fn cube(side: f64) -> f64 {
		return side * side * side;
	}

	pub fn parallelepiped(length: f64, width: f64, height: f64) -> f64 {
		return length * width * height;
	}

	pub fn prism(base: f64, height: f64) -> f64 {
		return base * height;
	}

	pub fn cylinder(radius: f64, height: f64) -> f64 {
		return std::f64::consts::PI * radius * radius * height;
	}

	pub fn cone(base: f64, height: f64) -> f64 {
		return (base * height) / 3.0;
	}

	pub fn sphere(radius: f64) -> f64 {
		return (4.0 / 3.0) * (std::f64::consts::PI * radius * radius * radius);
	}
}

pub mod statistics {
	pub fn average(numbers: &[f64]) -> f64 {
		return numbers.iter().sum::<f64>() as f64 / numbers.len() as f64;
	}

	pub fn mean(numbers: &[f64]) -> f64 {
		return numbers.iter().sum::<f64>() as f64 / numbers.len() as f64;
	}
}

pub mod math {
	pub fn factorial(number: u64) -> u64 {
		match number {
			0 => 1,
			1 => 1,
			_ => factorial(number - 1) * number,
		}
	}

	pub fn is_prime(number: u64) -> bool {
		let limit = (number as f64).sqrt() as u64 + 1;
		for i in 2..=limit {
			if number % i == 0 {
				return false;
			}
		}
		return true;
	}
}

pub mod geography {
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

		let central_angle_inner: f64 = ((delta_latitude / 2.0).sin())
			* ((delta_longitude / 2.0).sin())
			+ ((delta_longitude / 2.0).sin())
				* ((delta_longitude / 2.0).sin())
				* (start_latitude_in_radians.cos())
				* (end_latitude_in_radians.cos());

		let central_angle: f64 =
			2.0 * ((central_angle_inner.sqrt()).atan2((1.0 - central_angle_inner).sqrt()));

		return earth_radius * central_angle;
	}
}
