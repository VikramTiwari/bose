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
