use std::ops::Neg;

use num_traits::{One};

use crate::SquareMatrix;

use super::matrix_init;
use super::minor::Minor;

pub trait Adj: SquareMatrix
where
    Self::Output: SquareMatrix
{
    type Output;

    /// Returns the adjugate matrix of the given square matrix
    /// 
    /// adj(A)
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let a = [
    ///     [1.0, 2.0],
    ///     [3.0, 4.0]
    /// ];
    /// let aa = [
    ///     [4.0, -2.0],
    ///     [-3.0, 1.0]
    /// ];
    /// assert_eq!(a.adj(), aa);
    /// ```
    fn adj(&self) -> Self::Output;
}

impl<F: One> Adj for [[F; 1]; 1]
where
    Self: SquareMatrix
{
    type Output = Self;

    fn adj(&self) -> Self::Output
    {
        [[F::one()]]
    }
}

impl<F: Neg<Output = F> + Clone> Adj for [[F; 2]; 2]
where
    Self: SquareMatrix
{
    type Output = Self;

    fn adj(&self) -> Self::Output
    {
        [
            [self[1][1].clone(), -self[0][1].clone()],
            [-self[1][0].clone(), self[0][0].clone()]
        ]
    }
}

macro_rules! adj {
    ($i:expr) => {
        impl<F: One + Neg<Output = F>> Adj for [[F; $i]; $i]
        where
            Self: SquareMatrix + Minor<Output = F, Index = (usize, usize)>
        {
            type Output = Self;
        
            fn adj(&self) -> Self::Output
            {
                matrix_init(|r, c| if (r+c)%2 == 0 {F::one()} else {-F::one()}*self.minor((c, r)))
            }
        }
    };
}

adj!(3);
adj!(4);
adj!(5);
adj!(6);
adj!(7);
adj!(8);
adj!(9);
adj!(10);
adj!(11);
adj!(12);
adj!(13);
adj!(14);
adj!(15);
adj!(16);