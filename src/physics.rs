//! Physics
use super::constant::c;

/// Example
///
/// ```rust
/// let density = bose::physics::density(10.0, 30.1);
/// assert_eq!(density, 301.0);
/// ```
pub fn density(mass: f64, volume: f64) -> f64 {
    return mass * volume;
}

/// Example
///
/// ```rust
/// let pressure = bose::physics::pressure(30.0, 10.1);
/// assert_eq!(pressure, 2.9702970297029703);
/// ```
pub fn pressure(force: f64, area: f64) -> f64 {
    return force / area;
}

/// Example
///
/// ```rust
/// let mass = bose::physics::mass(10.0, 301.0);
/// assert_eq!(mass, 3010.0);
/// ```
pub fn mass(density: f64, volume: f64) -> f64 {
    return density * volume;
}

/// Example
///
/// ```rust
/// let energy = bose::physics::energy(10.0);
/// assert_eq!(energy, 8.987551787368177e17);
/// ```
pub fn energy(mass: f64) -> f64 {
    return mass * c.powf(2.0);
}

/// Example
///
/// ```rust
/// let speed = bose::physics::speed(10.0, 30.1);
/// assert_eq!(speed, 0.33222591362126247);
/// ```
pub fn speed(distance: f64, time: f64) -> f64 {
    return distance / time;
}

/// Example
///
/// ```rust
/// let work = bose::physics::work(10.0, 30.1);
/// assert_eq!(work, 301.0);
/// ```
pub fn work(force: f64, distance: f64) -> f64 {
    return force * distance;
}

/// Example
///
/// ```rust
/// let momentum = bose::physics::momentum(10.0, 30.1);
/// assert_eq!(momentum, 301.0);
/// ```
pub fn momentum(mass: f64, velocity: f64) -> f64 {
    return mass * velocity;
}

/// Example
///
/// ```rust
/// let acceleration = bose::physics::acceleration(30.0, 10.1, 2.3);
/// assert_eq!(acceleration, 8.652173913043478);
/// ```
pub fn acceleration(final_velocity: f64, initial_velocity: f64, time: f64) -> f64 {
    return (final_velocity - initial_velocity) / time;
}

/// Example
///
/// ```rust
/// let force = bose::physics::force(30.0, 10.1);
/// assert_eq!(force, 303.0);
/// ```
pub fn force(mass: f64, acceleration: f64) -> f64 {
    return mass * acceleration;
}

/// Example
///
/// ```rust
/// let weight = bose::physics::weight(30.0, 9.8);
/// assert_eq!(weight, 294.0);
/// ```
pub fn weight(mass: f64, gravity: f64) -> f64 {
    return mass * gravity;
}

/// Example
///
/// ```rust
/// let power = bose::physics::power(30.0, 10.0);
/// assert_eq!(power, 3.0);
/// ```
pub fn power(work: f64, time: f64) -> f64 {
    return work / time;
}

/// Example
///
/// ```rust
/// let mechanical_energy = bose::physics::mechanical_energy(30.0, 10.1);
/// assert_eq!(mechanical_energy, 40.1);
/// ```
pub fn mechanical_energy(potential_energy: f64, kinetic_energy: f64) -> f64 {
    return potential_energy + kinetic_energy;
}

/// Example
///
/// ```rust
/// let mechanical_advantage = bose::physics::mechanical_advantage(30.0, 10.1);
/// assert_eq!(mechanical_advantage, 2.9702970297029703);
/// ```
pub fn mechanical_advantage(output_force: f64, input_force: f64) -> f64 {
    return output_force / input_force;
}

/// Example
///
/// ```rust
/// let gpe = bose::physics::gpe(30.0, 10.1, 20.4);
/// assert_eq!(gpe, 6181.2);
/// ```
pub fn gpe(mass: f64, gravity: f64, height: f64) -> f64 {
    return mass * gravity * height;
}

/// Example
///
/// ```rust
/// let ke = bose::physics::ke(30.0, 10.1);
/// assert_eq!(ke, 1530.1499999999999);
/// ```
pub fn ke(mass: f64, velocity: f64) -> f64 {
    return mass * 0.5 * velocity * velocity;
}

/// Example
///
/// ```rust
/// let heat_energy = bose::physics::heat_energy(30.0, 10.1, 20.4);
/// assert_eq!(heat_energy, 3090.6);
/// ```
pub fn heat_energy(mass: f64, specific_heat_value: f64, change_in_temperature: f64) -> f64 {
    return mass * 0.5 * specific_heat_value * change_in_temperature;
}

/// Example
///
/// ```rust
/// let wave_speed = bose::physics::wave_speed(30.0, 10.1);
/// assert_eq!(wave_speed, 303.0);
/// ```
pub fn wave_speed(frequency: f64, wavelength: f64) -> f64 {
    return frequency * wavelength;
}

/// Example
///
/// ```rust
/// let current = bose::physics::current(30.0, 10.1);
/// assert_eq!(current, 2.9702970297029703);
/// ```
pub fn current(voltage: f64, resistance: f64) -> f64 {
    return voltage / resistance;
}

/// Example
///
/// ```rust
/// let efficiency = bose::physics::efficiency(30.0, 10.1);
/// assert_eq!(efficiency, 297.02970297029703);
/// ```
pub fn efficiency(output: f64, input: f64) -> f64 {
    return (output / input) * 100.0;
}

/// Example
///
/// ```rust
/// let electric_power = bose::physics::electric_power(30.0, 10.1);
/// assert_eq!(electric_power, 303.0);
/// ```
pub fn electric_power(current: f64, voltage: f64) -> f64 {
    return current * voltage;
}

/// Example
///
/// ```rust
/// let electric_variation = bose::physics::electric_variation(30.0, 10.1);
/// assert_eq!(electric_variation, 89.10891089108911);
/// ```
pub fn electric_variation(voltage: f64, resistance: f64) -> f64 {
    return (voltage * voltage) / resistance;
}

/// Example
///
/// ```rust
/// let celcius_to_kelvin = bose::physics::celcius_to_kelvin(30.0);
/// assert_eq!(celcius_to_kelvin, 303.15);
/// ```
pub fn celcius_to_kelvin(celcius: f64) -> f64 {
    return celcius + 273.15;
}

/// Example
///
/// ```rust
/// let kelvin_to_celcius = bose::physics::kelvin_to_celcius(30.0);
/// assert_eq!(kelvin_to_celcius, -243.14999999999998);
/// ```
pub fn kelvin_to_celcius(kelvin: f64) -> f64 {
    return kelvin - 273.15;
}

/// Example
///
/// ```rust
/// let celcius_to_fahrenheit = bose::physics::celcius_to_fahrenheit(30.0);
/// assert_eq!(celcius_to_fahrenheit, 86.0);
/// ```
pub fn celcius_to_fahrenheit(celcius: f64) -> f64 {
    return (1.8 * celcius) + 32.0;
}

/// Example
///
/// ```rust
/// let fahrenheit_to_celcius = bose::physics::fahrenheit_to_celcius(30.0);
/// assert_eq!(fahrenheit_to_celcius, -1.1111111111111112);
/// ```
pub fn fahrenheit_to_celcius(fahrenheit: f64) -> f64 {
    return (fahrenheit - 32.0) / 1.8;
}

/// Ideal mechanical advantage
///
/// Example
///
/// ```rust
/// let ima = bose::physics::ima(30.0, 10.1);
/// assert_eq!(ima, 2.9702970297029703);
/// ```
pub fn ima(length: f64, height: f64) -> f64 {
    return length / height;
}
