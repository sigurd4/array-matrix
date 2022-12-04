use std::ops::Add;

use crate::{Vector};

pub trait VAdd<Rhs: Vector>: Vector
where
    Self::Output: Vector
{
    type Output;

    /// Adds two vector-arrays of equal dimensions
    /// 
    /// u + v
    /// 
    /// # Arguments
    /// 
    /// * `rhs` - The addend vector
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let u = [1.0, 2.0];
    /// let v = [3.0, 4.0];
    /// let w = [u[0] + v[0], u[1] + v[1]];
    /// assert_eq!(u.add(v), w);
    /// ```
    fn add(self, rhs: Rhs) -> Self::Output;
}

impl<T1, T2, const L: usize> VAdd<[T2; L]> for [T1; L]
where
    Self: Vector,
    [T2; L]: Vector,
    [<T1 as Add<T2>>::Output; L]: Vector,
    T1: Add<T2> + Clone,
    T2: Clone
{
    type Output = [<T1 as Add<T2>>::Output; L];

    fn add(self, rhs: [T2; L]) -> Self::Output
    {
        array_init::array_init(|i| self[i].clone() + rhs[i].clone())
    }
}