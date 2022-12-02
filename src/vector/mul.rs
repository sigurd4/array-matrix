use std::ops::Mul;

use crate::Vector;

pub trait VMul<Rhs>: Vector
where Self::Output: Vector
{
    type Output;

    /// Returns the product of a vector-array and a scalar
    /// 
    /// ua
    /// 
    /// # Arguments
    /// 
    /// * `rhs` - A scalar
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let u = [1.0, 2.0];
    /// let a = 2.0
    /// let ua = [u[0]*a, u[1]*a];
    /// assert_eq!(u.mul(a), ua);
    /// ```
    fn mul(&self, rhs: Rhs) -> Self::Output;
}

impl<F, R, const N: usize> VMul<R> for [F; N]
where
    Self: Vector,
    [<F as Mul<R>>::Output; N]: Vector,
    F: Mul<R> + Copy,
    R: Copy
{
    type Output = [<F as Mul<R>>::Output; N];
    fn mul(&self, rhs: R) -> Self::Output
    {
        self.map(|x| x*rhs)
    }
}