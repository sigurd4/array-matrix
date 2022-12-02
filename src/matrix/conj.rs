use num_complex::{Complex};
use num_traits::Float;

use crate::{matrix_init, Matrix};

pub trait MConj: Matrix
where
    Self::Output: Matrix
{
    type Output;

    /// Returns the complex-conjugate matrix
    /// 
    /// A*
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let a = [
    ///     [Complex::new(1.0, 1.0), Complex::new(2.0, -1.0)],
    ///     [Complex::new(3.0, 1.0), Complex::new(4.0, -1.0)]
    /// ];
    /// let a_ = [
    ///     [a[0][0].conj(), a[0][1].conj()],
    ///     [a[1][0].conj(), a[1][1].conj()]
    /// ];
    /// assert_eq!(a.conj(), a_);
    /// ```
    fn conj(&self) -> Self::Output;
}

impl<F: Float, const L: usize, const H: usize> MConj for [[Complex<F>; L]; H]
where
    Self: Matrix
{
    type Output = Self;
    fn conj(&self) -> Self::Output
    {
        matrix_init(|r, c| self[r][c].conj())
    }
}