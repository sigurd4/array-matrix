use std::ops::Add;

use crate::{Det, Vector};

pub trait Cross<Rhs: Vector>: Vector
where Self::Output: Vector
{
    type Output;

    /// Returns the cross product of a 3- or 7-dimensional vector pair.
    /// 
    /// A×B
    /// 
    /// # Arguments
    /// 
    /// * `rhs` - Vector of same dimensions
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let a = [1.0, 0.0, 0.0];
    /// let b = [0.0, 1.0, 0.0];
    /// let ab = [0.0, 0.0, 1.0];
    /// assert_eq!(a.cross(b), ab);
    /// ```
    fn cross(self, rhs: Rhs) -> Self::Output;
}

impl<F> Cross<[F; 3]> for [F; 3]
where 
    Self: Vector,
    [<[[F; 2]; 2] as Det>::Output; 3]: Vector,
    [[F; 2]; 2]: Det,
    F: Copy
{
    type Output = [<[[F; 2]; 2] as Det>::Output; 3];
    fn cross(self, rhs: [F; 3]) -> Self::Output {
        use array_init::array_init;
        array_init(|i| [
            [self[(1 + i)%3], self[(2 + i)%3]],
            [rhs[(1 + i)%3], rhs[(2 + i)%3]]
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
    F: Copy
{
    type Output = [
        <<<[[F; 2]; 2] as Det>::Output
            as Add<<[[F; 2]; 2] as Det>::Output>>::Output
                as Add<<[[F; 2]; 2] as Det>::Output>>::Output; 7];
    fn cross(self, rhs: [F; 7]) -> Self::Output {
        use array_init::array_init;
        array_init(|i| [
            [self[(1 + i)%7], self[(3 + i)%7]],
            [rhs[(1 + i)%7], rhs[(3 + i)%7]]
        ].det() + [
            [self[(2 + i)%7], self[(6 + i)%7]],
            [rhs[(2 + i)%7], rhs[(6 + i)%7]]
        ].det() + [
            [self[(4 + i)%7], self[(5 + i)%7]],
            [rhs[(4 + i)%7], rhs[(5 + i)%7]]
        ].det())
    }
}