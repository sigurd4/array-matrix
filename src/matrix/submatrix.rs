use crate::matrix::matrix_init;

use super::Matrix;

pub trait Submatrix<O>: Matrix
where
    O: Matrix
{
    type Index;
    /// Returns the submatrix of a given cell
    /// 
    /// Mᵢⱼ
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
    /// let s = [
    ///     [5.0, 6.0],
    ///     [8.0, 9.0]
    /// ];
    /// assert_eq!(a.submatrix((0, 0)), s);
    /// ```
    fn submatrix(&self, index: Self::Index) -> O;
}


impl<F: Copy, const L: usize, const H: usize> Submatrix<[[F; L - 1]; H]> for [[F; L]; H]
where
    Self: Matrix,
    [[F; L - 1]; H]: Matrix
{
    type Index = usize;

    fn submatrix(&self, r: usize) -> [[F; L - 1]; H]
    {
        matrix_init(|i, j| self[(i + r + 1)%H][j])
    }
}

impl<F: Copy, const L: usize, const H: usize> Submatrix<[[F; L]; H - 1]> for [[F; L]; H]
where
    Self: Matrix,
    [[F; L]; H - 1]: Matrix
{
    type Index = usize;

    fn submatrix(&self, c: usize) -> [[F; L]; H - 1]
    {
        matrix_init(|i, j| self[i][(j + c + 1)%L])
    }
}

impl<F: Copy, const L: usize, const H: usize> Submatrix<[[F; L - 1]; H - 1]> for [[F; L]; H]
where
    Self: Matrix,
    [[F; L - 1]; H - 1]: Matrix
{
    type Index = (usize, usize);

    fn submatrix(&self, rc: (usize, usize)) -> [[F; L - 1]; H - 1]
    {
        matrix_init(|i, j| self[(i + rc.0 + 1)%H][(j + rc.1 + 1)%L])
    }
}