use std::ops::Mul;

use crate::{matrix_init, Matrix};

pub trait Outer<Rhs>
where Self::Output: Matrix
{
    type Output;
    
    /// Returns the outer product of two vector arrays
    /// 
    /// u ⊗ v
    /// 
    /// # Arguments
    /// 
    /// * `rhs` - A vector
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let u = [1.0, 2.0, 3.0];
    /// let v = [1.0, 2.0, 3.0];
    /// let uv = [
    ///     [1.0, 2.0, 3.0],
    ///     [2.0, 4.0, 6.0],
    ///     [3.0, 6.0, 9.0]
    /// ];
    /// 
    /// assert_eq!(u.outer(v), uv);
    /// ```
    fn outer(self, rhs: Rhs) -> Self::Output;
}

impl<F, const L: usize, const H: usize> Outer<[F; L]> for [F; H]
where
    [[<F as Mul<F>>::Output; L]; H]: Matrix,
    F: Copy + Mul<F>
{
    type Output = [[<F as Mul<F>>::Output; L]; H];
    fn outer(self, rhs: [F; L]) -> Self::Output
    {
        matrix_init(|r, c| self[r]*rhs[c])
    }
}