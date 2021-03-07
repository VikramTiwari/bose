use bose;

#[test]
fn areas() {
	let square = bose::areas::square(10.0);
	assert_eq!(100.0, square);

	let rectangle = bose::areas::rectangle(10.0, 5.0);
	assert_eq!(50.0, rectangle);

	let triangle = bose::areas::triangle(10.0, 5.0);
	assert_eq!(25.0, triangle);

	let rhombus = bose::areas::rhombus(10.0, 10.0);
	assert_eq!(50.0, rhombus);

	let trapezoid = bose::areas::trapezoid(10.0, 5.0, 2.0);
	assert_eq!(15.0, trapezoid);

	let polygon = bose::areas::polygon(10.0, 5.0);
	assert_eq!(25.0, polygon);

	let circle = bose::areas::circle(10.0);
	assert_eq!(314.1592653589793, circle);

	let cone = bose::areas::cone(10.0, 5.0);
	assert_eq!(157.07963267948966, cone);

	let sphere = bose::areas::sphere(10.0);
	assert_eq!(1256.6370614359173, sphere);
}
