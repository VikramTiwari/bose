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
