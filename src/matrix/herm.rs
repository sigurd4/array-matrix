use num_complex::{Complex};
use num_traits::Float;

use crate::{matrix_init, Matrix};

pub trait Herm: Matrix
where
    Self::Output: Matrix
{
    type Output;
    
    /// Returns the Hermitian complex-conjugate transposed matrix
    /// 
    /// Aᴴ = (A*)ᵀ
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let a = [
    ///     [Complex::new(1.0, 1.0), Complex::new(2.0, -1.0)],
    ///     [Complex::new(3.0, 1.0), Complex::new(4.0, -1.0)]
    /// ];
    /// let ah = [
    ///     [a[0][0].conj(), a[1][0].conj()],
    ///     [a[0][1].conj(), a[1][1].conj()]
    /// ];
    /// assert_eq!(a.herm(), ah);
    /// ```
    fn herm(&self) -> Self::Output;
}

impl<F: Float, const L: usize, const H: usize> Herm for [[Complex<F>; L]; H]
where
    Self: Matrix,
    [[Complex<F>; H]; L]: Matrix
{
    type Output = [[Complex<F>; H]; L];
    fn herm(&self) -> Self::Output
    {
        matrix_init(|r, c| self[c][r].conj())
    }
}