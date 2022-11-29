use crate::Matrix;

use super::det::Det;
use super::submatrix::Submatrix;

pub trait Minor: Matrix
{
    type Index;
    type Output;

    /// Returns the determinant of the submatrix of a given cell
    /// 
    /// |Mᵢⱼ|
    /// 
    /// # Arguments
    /// 
    /// * `index` - Row and/or collumn to neglect
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let a = [
    ///     [1.0, 2.0, 3.0],
    ///     [4.0, 5.0, 6.0],
    ///     [7.0, 8.0, 9.0]
    /// ];
    /// let m = [
    ///     [5.0, 6.0],
    ///     [8.0, 9.0]
    /// ].det();
    /// assert_eq!(a.minor((0, 0)), m);
    /// ```
    fn minor(&self, index: Self::Index) -> Self::Output;
}

impl<F, const N: usize> Minor for [[F; N]; N]
where
    Self: Submatrix<[[F; N - 1]; N - 1], Index = (usize, usize)>,
    [[F; N - 1]; N - 1]: Det
{
    type Index = <Self as Submatrix<[[F; N - 1]; N - 1]>>::Index;
    type Output = <[[F; N - 1]; N - 1] as Det>::Output;
    fn minor(&self, index: Self::Index) -> Self::Output
    {
        self.submatrix(index).det()
    }
}

/*impl<F, const N: usize> Minor for [[F; N - 1]; N]
where
    Self: Submatrix<[[F; N - 1]; N - 1], Index = usize>,
    [[F; N - 1]; N - 1]: Det
{
    type Index = <Self as Submatrix<[[F; N - 1]; N - 1]>>::Index;
    type Output = <[[F; N - 1]; N - 1] as Det>::Output;
    fn minor(&self, index: Self::Index) -> Self::Output
    {
        self.submatrix(index).det()
    }
}

impl<F, const N: usize> Minor for [[F; N]; N - 1]
where
    Self: Submatrix<[[F; N - 1]; N - 1], Index = usize>,
    [[F; N - 1]; N - 1]: Det
{
    type Index = <Self as Submatrix<[[F; N - 1]; N - 1]>>::Index;
    type Output = <[[F; N - 1]; N - 1] as Det>::Output;
    fn minor(&self, index: Self::Index) -> Self::Output
    {
        self.submatrix(index).det()
    }
}*/