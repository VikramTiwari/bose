use bose;

#[test]
fn area() {
	let square = bose::area::square(10.0);
	assert_eq!(100.0, square);

	let rectangle = bose::area::rectangle(10.0, 5.0);
	assert_eq!(50.0, rectangle);

	let triangle = bose::area::triangle(10.0, 5.0);
	assert_eq!(25.0, triangle);

	let rhombus = bose::area::rhombus(10.0, 10.0);
	assert_eq!(50.0, rhombus);

	let trapezoid = bose::area::trapezoid(10.0, 5.0, 2.0);
	assert_eq!(15.0, trapezoid);

	let polygon = bose::area::polygon(10.0, 5.0);
	assert_eq!(25.0, polygon);

	let circle = bose::area::circle(10.0);
	assert_eq!(314.1592653589793, circle);

	let cone = bose::area::cone(10.0, 5.0);
	assert_eq!(157.07963267948966, cone);

	let sphere = bose::area::sphere(10.0);
	assert_eq!(1256.6370614359173, sphere);
}

#[test]
fn perimeter() {
	let circle = bose::perimeter::circle(10.0);
	assert_eq!(62.83185307179586, circle);
}

#[test]
fn volume() {
	let cube = bose::volume::cube(10.0);
	assert_eq!(1000.0, cube);

	let parallelepiped = bose::volume::parallelepiped(10.0, 5.0, 2.0);
	assert_eq!(100.0, parallelepiped);

	let prism = bose::volume::prism(10.0, 5.0);
	assert_eq!(50.0, prism);

	let cylinder = bose::volume::cylinder(10.0, 5.0);
	assert_eq!(1570.7963267948967, cylinder);

	let cone = bose::volume::cone(10.0, 5.0);
	assert_eq!(16.666666666666668, cone);

	let sphere = bose::volume::sphere(10.0);
	assert_eq!(4188.790204786391, sphere);
}

#[test]
fn statistics() {
	let average = bose::statistics::average(&[10.0, 20.0, 30.0, 40.0, 50.0]);
	assert_eq!(30.0, average);

	let mean = bose::statistics::mean(&[10.0, 20.0, 30.0, 40.0, 50.0]);
	assert_eq!(30.0, mean);
}

#[test]
fn math() {
	let factorial = bose::math::factorial(10);
	assert_eq!(3628800, factorial);

	let is_prime = bose::math::is_prime(53);
	assert_eq!(true, is_prime);

	let is_not_prime = bose::math::is_prime(153);
	assert_eq!(false, is_not_prime);
}
