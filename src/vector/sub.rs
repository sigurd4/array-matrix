use std::ops::Add;

use crate::{Vector};

pub trait VSub<Rhs: Vector>: Vector
where
    Self::Output: Vector
{
    type Output;

    /// Subtracts two vector-arrays of equal dimensions
    /// 
    /// u - v
    /// 
    /// # Arguments
    /// 
    /// * `rhs` - The vector to subtract
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let u = [1.0, 2.0];
    /// let v = [3.0, 4.0];
    /// let w = [u[0] - v[0], u[1] - v[1]];
    /// assert_eq!(u.add(v), w);
    /// ```
    fn sub(self, rhs: Rhs) -> Self::Output;
}

impl<T1, T2, const L: usize> VSub<[T2; L]> for [T1; L]
where
    Self: Vector,
    [T2; L]: Vector,
    [<T1 as Add<T2>>::Output; L]: Vector,
    T1: Add<T2> + Copy,
    T2: Copy
{
    type Output = [<T1 as Add<T2>>::Output; L];

    fn sub(self, rhs: [T2; L]) -> Self::Output
    {
        array_init::array_init(|i| self[i] + rhs[i])
    }
}