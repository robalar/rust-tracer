use std::ops::{Add, Mul};

use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Colour<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T> Colour<T>
where
    rand::distributions::Standard: rand::distributions::Distribution<T>,
{
    pub fn random() -> Colour<T> {
        let mut rng = rand::thread_rng();
        Colour {
            r: rng.gen(),
            g: rng.gen(),
            b: rng.gen(),
        }
    }
}

impl From<Colour<f64>> for Colour<u32> {
    fn from(item: Colour<f64>) -> Colour<u32> {
        Colour {
            r: (256.0 * item.r.clamp(0.0, 0.999)) as u32,
            g: (256.0 * item.g.clamp(0.0, 0.999)) as u32,
            b: (256.0 * item.b.clamp(0.0, 0.999)) as u32,
        }
    }
}

impl<T: Mul<Output = T> + Copy> Colour<T> {
    pub(crate) fn new(r: T, g: T, b: T) -> Self {
        Colour { r, g, b }
    }

    pub fn mul_element_wise(&self, other: Colour<T>) -> Colour<T> {
        Colour::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy> Add for Colour<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Colour::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Colour<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Colour::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}
