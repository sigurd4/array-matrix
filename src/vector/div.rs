use std::ops::Div;

use crate::Vector;

pub trait VDiv<Rhs>: Vector
where Self::Output: Vector
{
    type Output;

    /// Returns the vector-array divided by a scalar
    /// 
    /// u/b
    /// 
    /// # Arguments
    /// 
    /// * `rhs` - A scalar
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let u = [1.0, 2.0];
    /// let b = 2.0
    /// let u_b = [u[0]/b, u[1]/b];
    /// assert_eq!(u.div(b), u_b);
    /// ```
    fn div(&self, rhs: Rhs) -> Self::Output;
}

impl<F, R, const N: usize> VDiv<R> for [F; N]
where
    Self: Vector,
    [<F as Div<R>>::Output; N]: Vector,
    F: Div<R> + Clone,
    R: Clone
{
    type Output = [<F as Div<R>>::Output; N];
    fn div(&self, rhs: R) -> Self::Output
    {
        array_init::array_init(|i| self[i].clone()/rhs.clone())
    }
}