use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// A 2D vector
#[derive(Copy, Clone, PartialEq, PartialOrd)]
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

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    /// Creates a vector with the given x coordinate and y = 0
    pub fn new_x(x: f32) -> Self {
        Self { x, y: 0.0 }
    }
    /// Creates a vector with the given y coordinate and x = 0
    pub fn new_y(y: f32) -> Self {
        Self { x: 0.0, y }
    }
    /// Creates a normalized vector with the angle in radians
    pub fn from_angle(angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::new(cos, sin)
    }
    /// Creates a vector with the given length and angle in radians
    pub fn from_polar(length: f32, angle: f32) -> Self {
        Self::from_angle(angle) * length
    }
    /// Returns the length of the vector
    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    /// Returns the squared length of the vector
    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y
    }
    /// Returns a vector with the same direction but with length 1
    pub fn normalized(self) -> Self {
        if self == Self::ZERO {
            Self::ZERO
        } else {
            self / self.length()
        }
    }
    /// Returns the dot product of two vectors
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }
    /// Sets the length of the vector
    pub fn set_length(&mut self, length: f32) {
        *self *= length / self.length();
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
impl Mul for Vec2 {
    type Output = Self;
    fn mul(self, rhs: Vec2) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}
impl Div for Vec2 {
    type Output = Self;
    fn div(self, rhs: Vec2) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y)
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
impl MulAssign for Vec2 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
impl DivAssign for Vec2 {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
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
