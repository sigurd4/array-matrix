use num_complex::Complex;
use num_traits::Float;

use crate::Vector;

pub trait VConj: Vector
where
    Self::Output: Vector
{
    type Output;

    /// Returns the complex-conjugate vector-array
    /// 
    /// u*
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let u = [Complex::new(1.0, 1.0), Complex::new(2.0, -1.0)];
    /// let u_ = [u[0].conj(), u[1].conj()];
    /// assert_eq!(u, u_);
    /// ```
    fn conj(&self) -> Self::Output;
}

impl<F: Float, const L: usize> VConj for [Complex<F>; L]
where
    Self: Vector
{
    type Output = Self;
    fn conj(&self) -> Self::Output
    {
        self.map(|x| x.conj())
    }
}