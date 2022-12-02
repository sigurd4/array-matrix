use std::ops::Sub;

use crate::Matrix;

use super::matrix_init;

pub trait MSub<Rhs: Matrix>: Matrix
where
    Self::Output: Matrix
{
    type Output;

    /// Subtracts one matrix from another of equal dimensions
    /// 
    /// A - B
    /// 
    /// # Arguments
    /// 
    /// * `rhs` - The subtracted matrix
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let a = [
    ///     [1.0, 2.0],
    ///     [3.0, 4.0]
    /// ];
    /// let b = [
    ///     [4.0, 3.0],
    ///     [2.0, 1.0]
    /// ];
    /// let s = [
    ///     [a[0][0] - b[0][0], a[0][1] - b[0][1]],
    ///     [a[1][0] - b[1][0], a[1][1] - b[1][1]]
    /// ];
    /// assert_eq!(a.sub(b), s);
    /// ```
    fn sub(self, rhs: Rhs) -> Self::Output;
}

impl<T1, T2, const H: usize, const L: usize> MSub<[[T2; L]; H]> for [[T1; L]; H]
where
    Self: Matrix,
    [[T2; L]; H]: Matrix,
    [[<T1 as Sub<T2>>::Output; L]; H]: Matrix,
    T1: Sub<T2> + Copy,
    T2: Copy
{
    type Output = [[<T1 as Sub<T2>>::Output; L]; H];

    fn sub(self, rhs: [[T2; L]; H]) -> Self::Output
    {
        matrix_init(|r, c| self[r][c] - rhs[r][c])
    }
}