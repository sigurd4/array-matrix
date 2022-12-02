use std::ops::{Div};

use crate::{matrix_init, Matrix};

pub trait MDiv<Rhs>: Matrix
where
    Self::Output: Matrix
{
    type Output;
    
    /// Returns matrix divided by a scalar
    /// 
    /// A/b
    /// 
    /// # Arguments
    /// 
    /// * `rhs` - A scalar
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let a = [
    ///     [1.0, 2.0],
    ///     [3.0, 4.0]
    /// ];
    /// let b = 2.0;
    /// let a_b = [
    ///     [0.5, 1.0],
    ///     [1.5, 2.0]
    /// ];
    /// assert_eq!(a.div(b), a_b)
    /// ```
    fn div(self, rhs: Rhs) -> Self::Output;
}

impl<F, const L: usize, const H: usize> MDiv<F> for [[F; L]; H]
where
    Self: Matrix,
    [[<F as Div<F>>::Output; L]; H]: Matrix,
    F: Copy + Div<F>
{
    type Output = [[<F as Div<F>>::Output; L]; H];
    fn div(self, rhs: F) -> Self::Output
    {
        matrix_init(|r, c| self[r][c]/rhs)
    }
}