use crate::{SquareMatrix, Transpose, Adj};

pub trait Cofactor: SquareMatrix
where
    Self::Output: SquareMatrix
{
    type Output;

    /// Returns the cofactor matrix of the given matrix
    /// 
    /// adj(A)ᵀ
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let a = [
    ///     [1.0, 2.0],
    ///     [3.0, 4.0]
    /// ];
    /// let ac = [
    ///     [4.0, -3.0],
    ///     [-2.0, 1.0]
    /// ];
    /// assert_eq!(a.cofactor(), ac);
    /// ```
    fn cofactor(&self) -> Self::Output;
}

impl<M> Cofactor for M
where
    Self: Adj,
    <Self as Adj>::Output: Transpose,
    <<Self as Adj>::Output as Transpose>::Output: SquareMatrix
{
    type Output = <<Self as Adj>::Output as Transpose>::Output;
    fn cofactor(&self) -> Self::Output
    {
        self.adj().transpose()
    }
}