use std::ops::{Mul, Add};

use num_traits::Zero;

pub trait Dot<Rhs>
{
    type Output;

    /// Returns the dot product of two vector arrays
    /// 
    /// u ⋅ v
    /// 
    /// # Arguments
    /// 
    /// * `rhs` - A vector of same length
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let u = [1.0, 2.0, 3.0];
    /// let v = [1.0, 2.0, 3.0];
    /// let uv = 1.0 + 4.0 + 9.0;
    /// 
    /// assert_eq!(u.dot(v), uv);
    /// ```
    fn dot(self, rhs: Rhs) -> Self::Output;
}

impl<F, const L: usize> Dot<[F; L]> for [F; L]
where
    F: Copy + Mul<F, Output = F> + Add<F, Output = F> + Zero
{
    type Output = F;
    fn dot(self, rhs: [F; L]) -> Self::Output
    {
        (0..L).map(|i| self[i]*rhs[i]).reduce(|a, b| a + b).unwrap_or(F::zero())
    }
}