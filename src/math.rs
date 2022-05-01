//! Math

/// Example
///
/// ```rust
/// let factorial = bose::math::factorial(10);
/// assert_eq!(3628800, factorial);
/// ```
pub fn factorial(number: u64) -> u64 {
    match number {
        0 => 1,
        1 => 1,
        _ => factorial(number - 1) * number,
    }
}

/// Example
///
/// Is prime
/// ```rust
/// let is_prime = bose::math::is_prime(53);
/// assert_eq!(true, is_prime);
/// ```
///
/// Not prime
/// ```rust
/// let is_not_prime = bose::math::is_prime(153);
/// assert_eq!(false, is_not_prime);
/// ```
pub fn is_prime(number: u64) -> bool {
    let limit = (number as f64).sqrt() as u64 + 1;
    for i in 2..=limit {
        if number % i == 0 {
            return false;
        }
    }
    return true;
}
