use num_complex::Complex;
use num_traits::Float;

use crate::Vector;

pub trait VAbs: Vector
{
    type Output;

    /// Returns the square euclidian absolute value of any vector-array
    /// 
    /// ||u||²
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let u = [1.0, 2.0];
    /// let u_abs_sqr = u[0]*u[0] + u[1]*u[1];
    /// 
    /// assert_eq!(u.abs_sqr(), u_abs_sqr);
    /// ```
    fn abs_sqr(&self) -> Self::Output;
    
    /// Returns the euclidian absolute value of any vector-array
    /// 
    /// ||u||
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let u = [1.0, 2.0];
    /// let u_abs = (u[0]*u[0] + u[1]*u[1]).sqrt();
    /// 
    /// assert_eq!(u.abs(), u_abs);
    /// ```
    fn abs(&self) -> Self::Output;
}

impl<const N: usize> VAbs for [f32; N]
{
    type Output = f32;
    fn abs_sqr(&self) -> Self::Output
    {
        self.iter().map(|x| x.abs().powi(2)).reduce(|a, b| a + b).unwrap_or(0.0)
    }
    fn abs(&self) -> Self::Output
    {
        self.abs_sqr().sqrt()
    }
}

impl<const N: usize> VAbs for [f64; N]
{
    type Output = f64;
    fn abs_sqr(&self) -> Self::Output
    {
        self.iter().map(|x| x.abs().powi(2)).reduce(|a, b| a + b).unwrap_or(0.0)
    }
    fn abs(&self) -> Self::Output
    {
        self.abs_sqr().sqrt()
    }
}

impl<F: Float, const N: usize> VAbs for [Complex<F>; N]
{
    type Output = F;
    fn abs_sqr(&self) -> Self::Output
    {
        self.iter().map(|x| x.norm_sqr()).reduce(|a, b| a + b).unwrap_or(F::zero())
    }
    fn abs(&self) -> Self::Output
    {
        self.abs_sqr().sqrt()
    }
}