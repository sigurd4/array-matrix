use std::ops::Mul;

use crate::{matrix_init, Matrix};

pub trait KroneckerMul<Rhs>: Matrix
where
    Rhs: Matrix,
    Self::Output: Matrix
{
    type Output;

    /// Returns the kronecker product of the two matrices
    /// 
    /// A ⊗ₖᵣₒₙ B
    /// 
    /// # Arguments
    /// 
    /// * `rhs` - A matrix of any size
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let a = [
    ///     [1.0, 2.0],
    ///     [3.0, 4.0]
    /// ];
    /// let b = [
    ///     [1.0, 2.0],
    ///     [3.0, 4.0]
    /// ];
    /// let ab = [
    ///     [1.0, 2.0, 2.0, 4.0],
    ///     [3.0, 4.0, 6.0, 8.0],
    ///     [3.0, 6.0, 4.0, 8.0],
    ///     [9.0, 12.0, 12.0, 16.0]
    /// ];
    /// assert_eq!(a.kronecker(b), ab);
    /// ```
    fn kronecker_mul(self, rhs: Rhs) -> Self::Output;
}

impl<F, const L1: usize, const H1: usize, const L2: usize, const H2: usize> 
    KroneckerMul<[[F; L2]; H2]>
for
    [[F; L1]; H1]
where
    Self: Matrix,
    [[F; L2]; H2]: Matrix,
    F: Copy + Mul<F>,
    [[<F as Mul<F>>::Output; L1*L2]; H1*H2]: Matrix
{
    type Output = [[<F as Mul<F>>::Output; L1*L2]; H1*H2];

    fn kronecker_mul(self, rhs: [[F; L2]; H2]) -> Self::Output
    {
        matrix_init(|r, c| self[r/H1][c/L1]*rhs[r%H2][c%L2])
    }
}