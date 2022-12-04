use std::ops::Add;

use crate::{Det, Vector};

pub trait Cross<Rhs: Vector>: Vector
where Self::Output: Vector
{
    type Output;

    /// Returns the cross product of a 3- or 7-dimensional vector pair.
    /// 
    /// u×v
    /// 
    /// # Arguments
    /// 
    /// * `rhs` - Vector of same dimensions
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let u = [1.0, 0.0, 0.0];
    /// let v = [0.0, 1.0, 0.0];
    /// let uv = [0.0, 0.0, 1.0];
    /// assert_eq!(u.cross(v), uv);
    /// ```
    fn cross(self, rhs: Rhs) -> Self::Output;
}

impl<F> Cross<[F; 3]> for [F; 3]
where 
    Self: Vector,
    [<[[F; 2]; 2] as Det>::Output; 3]: Vector,
    [[F; 2]; 2]: Det,
    F: Clone
{
    type Output = [<[[F; 2]; 2] as Det>::Output; 3];
    fn cross(self, rhs: [F; 3]) -> Self::Output {
        use array_init::array_init;
        array_init(|i| [
            [self[(1 + i)%3].clone(), self[(2 + i)%3].clone()],
            [rhs[(1 + i)%3].clone(), rhs[(2 + i)%3].clone()]
        ].det())
    }
}

impl<F> Cross<[F; 7]> for [F; 7]
where
    Self: Vector,
    [<<<[[F; 2]; 2] as Det>::Output
            as Add<<[[F; 2]; 2] as Det>::Output>>::Output
                as Add<<[[F; 2]; 2] as Det>::Output>>::Output; 7]: Vector,
    [[F; 2]; 2]: Det,
    <[[F; 2]; 2] as Det>::Output: Add,
    <<[[F; 2]; 2] as Det>::Output as Add<<[[F; 2]; 2] as Det>::Output>>::Output:
        Add<<[[F; 2]; 2] as Det>::Output>,
    F: Clone
{
    type Output = [
        <<<[[F; 2]; 2] as Det>::Output
            as Add<<[[F; 2]; 2] as Det>::Output>>::Output
                as Add<<[[F; 2]; 2] as Det>::Output>>::Output; 7];
    fn cross(self, rhs: [F; 7]) -> Self::Output {
        use array_init::array_init;
        array_init(|i| [
            [self[(1 + i)%7].clone(), self[(3 + i)%7].clone()],
            [rhs[(1 + i)%7].clone(), rhs[(3 + i)%7].clone()]
        ].det() + [
            [self[(2 + i)%7].clone(), self[(6 + i)%7].clone()],
            [rhs[(2 + i)%7].clone(), rhs[(6 + i)%7].clone()]
        ].det() + [
            [self[(4 + i)%7].clone(), self[(5 + i)%7].clone()],
            [rhs[(4 + i)%7].clone(), rhs[(5 + i)%7].clone()]
        ].det())
    }
}