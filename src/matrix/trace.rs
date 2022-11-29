use std::ops::Add;
use num_traits::Zero;

use super::SquareMatrix;

pub trait Trace: SquareMatrix
{
    type Output;

    /// Returns the trace of a given matrix
    /// 
    /// Tr(A)
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let a = [
    ///     [1.0, 2.0, 3.0],
    ///     [4.0, 5.0, 6.0],
    ///     [7.0, 8.0, 9.0]
    /// ];
    /// let t = a[0][0] + a[1][1] + a[2][2];
    /// assert_eq!(a.trace(), t);
    /// ```
    fn trace(&self) -> Self::Output;
}

impl<F, const N: usize> Trace for [[F; N]; N]
where
    Self: SquareMatrix,
    F: Add<F, Output = F> + Copy + Zero
{
    type Output = F;
    fn trace(&self) -> Self::Output
    {
        (0..N).map(|i| self[i][i]).reduce(|a, b| a + b).unwrap_or(F::zero())
    }
}