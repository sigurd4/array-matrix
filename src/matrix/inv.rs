use num_traits::Inv;
use num_traits::Zero;

use crate::MMul;
use crate::Adj;
use crate::Det;
use crate::SquareMatrix;

pub trait MInv: SquareMatrix
where
    Self::Output: SquareMatrix
{
    type Output;

    /// Returns the inverted matrix if the matrix is non-singular
    /// 
    /// A⁻¹
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// // Returns none if matrix is singular
    /// let a = [
    ///     [1.0, 2.0],
    ///     [2.0, 4.0]
    /// ];
    /// assert_eq!(a.inv(), None);
    /// 
    /// // Otherwise equals adjunct matrix divided by determinant
    /// let a = [
    ///     [1.0, 2.0],
    ///     [3.0, 4.0]
    /// ];
    /// let ai = a.adj().dot(a.det().recip());
    /// assert_eq!(a.inv(), Some(ai));
    /// ```
    fn inv(&self) -> Option<Self::Output>;
}

impl<F, M> MInv for M
where
    F: Inv<Output = F> + Zero,
    Self: Adj + Det<Output = F>,
    <Self as Adj>::Output: MMul<F>,
    <<Self as Adj>::Output as MMul<F>>::Output: SquareMatrix
{
    type Output = <<Self as Adj>::Output as MMul<F>>::Output;

    fn inv(&self) -> Option<Self::Output>
    {
        let det = self.det();
        if det.is_zero()
        {
            None
        }
        else
        {
            Some(self.adj().mul(det.inv()))
        }
    }
}