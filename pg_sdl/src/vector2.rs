use nalgebra::{Matrix2, Rotation2, Vector2};

const ROTATION90: Rotation2<f64> = Rotation2::from_matrix_unchecked(Matrix2::new(0.0, -1.0, 1.0, 0.0));

pub trait Vector2Plus {
	fn new_unitary(angle: f64) -> Self;
	fn new_polar(length: f64, angle: f64) -> Self;
	fn get_angle(&self) -> f64;
	fn perpendicular(&self) -> Self;
}
impl Vector2Plus for Vector2<f64> {
	/// Returns a new unitary vector with length 1 and a given angle in radians
	fn new_unitary(angle: f64) -> Self {
		let (sin, cos) = angle.sin_cos();
		Vector2::new(cos, sin)
	}
	/// Returns a new vector with a given length and angle in radians
	fn new_polar(length: f64, angle: f64) -> Self {
		Self::new_unitary(angle) * length
	}
	/// Returns the angle of the vector in radians form the x axis (0 - 2π)
	fn get_angle(&self) -> f64 {
		self.y.atan2(self.x)
	}
	/// Returns the normalized perpendicular vector (rotated 90° in the trigonometric direction)
	fn perpendicular(&self) -> Self {
		ROTATION90 * self.normalize()
	}
}

fn main() {
	let a = Vector2::new(2.0, 1.0);
	println!("{}", a.get_angle());
	println!("{}", a.perpendicular());
}
/*
use std::fmt;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A 2D vector
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Vec2 {
	pub x: f32,
	pub y: f32,
}

impl Vec2 {
	pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
	pub const X: Self = Self { x: 1.0, y: 0.0 };
	pub const Y: Self = Self { x: 0.0, y: 1.0 };

	pub fn as_point(self) -> sdl2::rect::Point {
		sdl2::rect::Point::new(self.x as i32, self.y as i32)
	}

	pub const fn new(x: f32, y: f32) -> Self {
		Self { x, y }
	}
	/// Creates a vector with the given x coordinate and y = 0
	pub const fn new_x(x: f32) -> Self {
		Self { x, y: 0.0 }
	}
	/// Creates a vector with the given y coordinate and x = 0
	pub const fn new_y(y: f32) -> Self {
		Self { x: 0.0, y }
	}
	/// Creates a vector with the same x and y coordinates
	pub const fn new_xy(xy: f32) -> Self {
		Self { x: xy, y: xy }
	}
	///Returns a new vector with the given x coordinate
	pub fn with_x(self, x: f32) -> Self {
		Self { x, ..self }
	}
	///Returns a new vector with the given y coordinate
	pub fn with_y(self, y: f32) -> Self {
		Self { y, ..self }
	}
	/// Creates a normalized vector with the angle in radians
	pub fn from_angle(angle: f32) -> Self {
		let (sin, cos) = angle.sin_cos();
		Self::new(cos, sin)
	}
	/// Creates a normalized vector with the angle in degrees
	pub fn from_angle_deg(angle: f32) -> Self {
		Self::from_angle(angle.to_radians())
	}
	/// Creates a vector with the given length and angle in radians
	pub fn from_polar(length: f32, angle: f32) -> Self {
		Self::from_angle(angle) * length
	}
	/// Creates a vector with the given length and angle in degrees
	pub fn from_polar_deg(length: f32, angle: f32) -> Self {
		Self::from_polar(length, angle.to_radians())
	}
	/// Returns a vector transformed by the given x and y director vectors
	pub fn linear_transform(self, x_dir: Self, y_dir: Self) -> Self {
		Self::new(self.x * x_dir.x + self.y * y_dir.x, self.x * x_dir.y + self.y * y_dir.y)
	}
	/// Returns a vector rotated by the angle of the given director vector
	pub fn rotation_transform(self, director: Self) -> Self {
		let normalized = self.normalized();
		self.linear_transform(normalized, normalized.perpendicular())
	}
	/// Returns a vector rotated and scaled by the given director vector
	pub fn rotation_scale_transform(self, director: Self) -> Self {
		self.linear_transform(director, director.perpendicular())
	}
	/// Returns the length of the vector
	pub fn length(self) -> f32 {
		(self.x * self.x + self.y * self.y).sqrt()
	}
	/// Returns the squared length of the vector
	pub fn length_squared(self) -> f32 {
		self.x * self.x + self.y * self.y
	}
	/// Sets the length of the vector to 1
	pub fn normalize(&mut self) {
		if *self != Self::ZERO {
			*self /= self.length();
		}
	}
	/// Returns a vector with the same direction but with length 1
	pub fn normalized(self) -> Self {
		if self == Self::ZERO {
			Self::ZERO
		} else {
			self / self.length()
		}
	}
	/// Returns a new vector normal to the vector (rotated 90 degrees in the trigonometric direction)
	pub fn perpendicular(self) -> Self {
		let normalized = self.normalized();
		Self::new(-normalized.y, normalized.x)
	}
	/// Returns the dot product of two vectors
	pub fn dot(self, other: Self) -> f32 {
		self.x * other.x + self.y * other.y
	}
	/// Returns the cross product of two vectors
	pub fn cross(self, other: Self) -> f32 {
		self.x * other.y - self.y * other.x
	}
	/// Returns the projection of the vector onto another vector
	pub fn projected_onto(self, other: Self) -> Self {
		if other == Self::ZERO {
			Self::ZERO
		} else {
			other * self.dot(other) / other.length_squared()
		}
	}
	/// Returns the angle between two vectors in radians
	pub fn angle_between(self, other: Self) -> f32 {
		// TODO: check if this is correct
		self.dot(other).atan2(self.cross(other))
	}
	/// Returns the distance between two vectors
	pub fn distance(self, other: Self) -> f32 {
		(self - other).length()
	}
	/// Sets the length of the vector
	pub fn set_length(&mut self, length: f32) {
		if *self == Self::ZERO {
			self.x = length;
		} else {
			*self *= length / self.length();
		}
	}
	/// Returns a new vector with the given length
	pub fn with_length(self, length: f32) -> Self {
		if self == Self::ZERO {
			Vec2::new_x(length)
		} else {
			self * length / self.length()
		}
	}
	/// Sets the angle of the vector in radians
	pub fn set_angle(&mut self, angle: f32) {
		let length = self.length();
		let (sin, cos) = angle.sin_cos();
		self.x = length * cos;
		self.y = length * sin;
	}
	/// Sets the angle of the vector in degrees
	pub fn set_angle_deg(&mut self, angle: f32) {
		self.set_angle(angle.to_radians());
	}
	/// Returns a new vector with the given angle in radians
	pub fn with_angle(self, angle: f32) -> Self {
		Self::from_polar_deg(self.length(), angle)
	}
	/// Returns a new vector with the given angle in degrees
	pub fn with_angle_deg(self, angle: f32) -> Self {
		self.with_angle(angle.to_radians())
	}
	/// Returns the angle of the vector in radians
	pub fn angle(self) -> f32 {
		self.y.atan2(self.x)
	}
	/// Returns the angle of the vector in degrees
	pub fn angle_deg(self) -> f32 {
		self.y.atan2(self.x).to_degrees()
	}
	/// Returns the angle between two vectors in radians
	pub fn angle_to(self, other: Self) -> f32 {
		(self - other).angle()
	}
	/// Returns the angle between two vectors in degrees
	pub fn angle_to_deg(self, other: Self) -> f32 {
		self.angle_to(other).to_degrees()
	}
	/// Returns a new vector with the same length but rotated by the given angle in radians
	pub fn rotated(self, angle: f32) -> Self {
		let (sin, cos) = angle.sin_cos();
		Self::new(self.x * cos - self.y * sin, self.x * sin + self.y * cos)
	}
	/// Returns a new vector with the same length but rotated by the given angle in degrees
	pub fn rotated_deg(self, angle: f32) -> Self {
		self.rotated(angle.to_radians())
	}
	/// Rotates the vector by the given angle in radians
	pub fn rotate(&mut self, angle: f32) {
		*self = self.rotated(angle);
	}
	/// Rotates the vector by the given angle in radians
	pub fn rotate_deg(&mut self, angle: f32) {
		*self = self.rotated_deg(angle);
	}
	/// Returns a new vector rotated by the given angle in radians around the given center
	pub fn rotated_around(self, angle: f32, center: Self) -> Self {
		(self - center).rotated(angle) + center
	}
	/// Returns a new vector rotated by the given angle in degrees around the given center
	pub fn rotated_around_deg(self, angle: f32, center: Self) -> Self {
		self.rotated_around(angle.to_radians(), center)
	}
	/// Rotates the vector by the given angle in radians around the given center
	pub fn rotate_around(&mut self, angle: f32, center: Self) {
		*self = self.rotated_around(angle, center);
	}
	/// Rotates the vector by the given angle in degrees around the given center
	pub fn rotate_around_deg(&mut self, angle: f32, center: Self) {
		*self = self.rotated_around_deg(angle, center);
	}
}

impl fmt::Display for Vec2 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Vec2({}, {})", self.x, self.y)
	}
}
impl Neg for Vec2 {
	type Output = Self;
	fn neg(self) -> Self::Output {
		Self::new(-self.x, -self.y)
	}
}

impl Add for Vec2 {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		Self::new(self.x + rhs.x, self.y + rhs.y)
	}
}
impl Sub for Vec2 {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {
		Self::new(self.x - rhs.x, self.y - rhs.y)
	}
}

impl AddAssign for Vec2 {
	fn add_assign(&mut self, rhs: Self) {
		*self = *self + rhs;
	}
}
impl SubAssign for Vec2 {
	fn sub_assign(&mut self, rhs: Self) {
		*self = *self - rhs;
	}
}

impl Mul<f32> for Vec2 {
	type Output = Self;
	fn mul(self, rhs: f32) -> Self::Output {
		Self::new(self.x * rhs, self.y * rhs)
	}
}
impl Div<f32> for Vec2 {
	type Output = Self;
	fn div(self, rhs: f32) -> Self::Output {
		Self::new(self.x / rhs, self.y / rhs)
	}
}

impl Mul<Vec2> for f32 {
	type Output = Vec2;
	fn mul(self, rhs: Vec2) -> Self::Output {
		Vec2::new(self * rhs.x, self * rhs.y)
	}
}

impl MulAssign<f32> for Vec2 {
	fn mul_assign(&mut self, rhs: f32) {
		*self = *self * rhs;
	}
}
impl DivAssign<f32> for Vec2 {
	fn div_assign(&mut self, rhs: f32) {
		*self = *self / rhs;
	}
}

impl From<sdl2::rect::Point> for Vec2 {
	fn from(point: sdl2::rect::Point) -> Self {
		Self::new(point.x() as f32, point.y() as f32)
	}
}
impl From<(f32, f32)> for Vec2 {
	fn from((x, y): (f32, f32)) -> Self {
		Self::new(x, y)
	}
}
impl From<[f32; 2]> for Vec2 {
	fn from([x, y]: [f32; 2]) -> Self {
		Self::new(x, y)
	}
}

impl Sum for Vec2 {
	fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
		iter.fold(Self::ZERO, |a, b| a + b)
	}
}

// Sum <&Vec2> for Vec2
*/
