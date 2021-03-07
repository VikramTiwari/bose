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
