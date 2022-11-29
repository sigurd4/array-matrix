use crate::Matrix;

use super::matrix_init;

pub trait Transpose: Matrix
where
    Self::Output: Matrix
{
    type Output;

    /// Returns the transposed version of the given matrix
    /// 
    /// Aᵀ
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let a = [
    ///     [1.0, 2.0, 3.0],
    ///     [4.0, 5.0, 6.0]
    /// ];
    /// let at = [
    ///     [1.0, 4.0],
    ///     [2.0, 5.0],
    ///     [3.0, 6.0]
    /// ];
    /// assert_eq!(a.transpose(), at);
    /// ```
    fn transpose(&self) -> Self::Output;
}

impl<F: Copy, const L: usize, const H: usize> Transpose for [[F; L]; H]
where
    Self: Matrix,
    [[F; H]; L]: Matrix
{
    type Output = [[F; H]; L];

    fn transpose(&self) -> Self::Output
    {
        matrix_init(|r, c| self[c][r])
    }
}

/*impl<F: Float, const L: usize> Transpose for [F; L]
{
    type Output = [[F; 1]; L];

    fn transpose(&self) -> Self::Output
    {
        matrix_init(|r, c| self[r])
    }
}*/