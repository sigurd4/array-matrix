use std::ops::{Mul, Add};

use crate::{matrix_init, Matrix};

pub trait MMul<Rhs>: Matrix
where
    Self::Output: Matrix
{
    type Output;
    
    /// Returns the matrix product
    /// 
    /// AB
    /// 
    /// # Arguments
    /// 
    /// * `rhs` - A scalar or a matrix with height equal this matrix's length
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// // Scaling
    /// let a = [
    ///     [1.0, 2.0],
    ///     [3.0, 4.0]
    /// ];
    /// let b = 2.0;
    /// let ab = [
    ///     [2.0, 4.0],
    ///     [6.0, 8.0]
    /// ];
    /// assert_eq!(a.mul(b), ab)
    /// 
    /// // Matrix multiplication
    /// let a = [
    ///     [1.0],
    ///     [2.0],
    ///     [3.0]
    /// ];
    /// let b = [
    ///     [1.0, 2.0, 3.0]
    /// ];
    /// let ab = [
    ///     [1.0, 2.0, 3.0],
    ///     [2.0, 4.0, 6.0],
    ///     [3.0, 6.0, 9.0]
    /// ];
    /// assert_eq!(a.mul(b), ab)
    /// ```
    fn mul(self, rhs: Rhs) -> Self::Output;
}

impl<F, const L: usize, const H: usize> MMul<F> for [[F; L]; H]
where
    Self: Matrix,
    [[<F as Mul<F>>::Output; L]; H]: Matrix,
    F: Copy + Mul<F>
{
    type Output = [[<F as Mul<F>>::Output; L]; H];
    fn mul(self, rhs: F) -> Self::Output
    {
        matrix_init(|r, c| self[r][c]*rhs)
    }
}

impl<F, const L: usize, const H1: usize, const H2: usize> MMul<[[F; H2]; L]>
for
    [[F; L]; H1]
where
    Self: Matrix,
    [[<F as Mul<F>>::Output; H2]; H1]: Matrix,
    F: Copy + Mul<F>,
    <F as Mul<F>>::Output: Add<<F as Mul<F>>::Output, Output = <F as Mul<F>>::Output>
{
    type Output = [[<F as Mul<F>>::Output; H2]; H1];
    fn mul(self, rhs: [[F; H2]; L]) -> Self::Output
    {
        matrix_init(|r, c| (0..L).map(|i| self[r][i]*rhs[i][c]).reduce(|a, b| a + b).unwrap())
    }
}