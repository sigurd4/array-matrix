pub trait Diag
{
    type Output;
    
    /// Returns the diagonal of the given matrix
    /// 
    /// {aᵢᵢ}
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let a = [
    ///     [1.0, 2.0],
    ///     [3.0, 4.0]
    /// ];
    /// assert_eq!(a.diag(), [1.0, 4.0]);
    /// ```
    fn diag(&self) -> Self::Output;
}

impl<F, const L: usize, const H: usize> Diag for [[F; L]; H]
where F: Clone
{
    type Output = Vec<F>;
    fn diag(&self) -> Self::Output
    {
        (0..L.min(H)).map(|i| self[i][i].clone()).collect()
    }
}