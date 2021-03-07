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
