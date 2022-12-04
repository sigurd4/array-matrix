use std::ops::Add;

use crate::Matrix;

use super::matrix_init;

pub trait MAdd<Rhs: Matrix>: Matrix
where
    Self::Output: Matrix
{
    type Output;

    /// Adds two matrices of equal dimensions
    /// 
    /// A + B
    /// 
    /// # Arguments
    /// 
    /// * `rhs` - The addend matrix
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
    ///     [a[0][0] + b[0][0], a[0][1] + b[0][1]],
    ///     [a[1][0] + b[1][0], a[1][1] + b[1][1]]
    /// ];
    /// assert_eq!(a.add(b), s);
    /// ```
    fn add(self, rhs: Rhs) -> Self::Output;
}

impl<T1, T2, const H: usize, const L: usize> MAdd<[[T2; L]; H]> for [[T1; L]; H]
where
    Self: Matrix,
    [[T2; L]; H]: Matrix,
    [[<T1 as Add<T2>>::Output; L]; H]: Matrix,
    T1: Add<T2> + Clone,
    T2: Clone
{
    type Output = [[<T1 as Add<T2>>::Output; L]; H];

    fn add(self, rhs: [[T2; L]; H]) -> Self::Output
    {
        matrix_init(|r, c| self[r][c].clone() + rhs[r][c].clone())
    }
}