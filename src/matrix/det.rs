use std::ops::{Mul, Sub, Add};

use crate::SquareMatrix;

use super::Matrix;
use super::minor::Minor;

pub trait Det: SquareMatrix
{
    type Output;

    /// Returns the determinant of the given matrix
    /// 
    /// |A|
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let a = [
    ///     [1.0, 0.0],
    ///     [0.0, 1.0]
    /// ];
    /// assert_eq!(a.det(), 1.0);
    /// ```
    fn det(&self) -> Self::Output;
}

impl<F: Copy> Det for [[F; 1]; 1]
where Self: SquareMatrix
{
    type Output = F;

    fn det(&self) -> Self::Output
    {
        self[0][0]
    }
}

impl<F: Copy + Mul<F, Output = F> + Sub<F, Output = F>> Det for [[F; 2]; 2]
where Self: SquareMatrix
{
    type Output = F;

    fn det(&self) -> Self::Output
    {
        self[0][0]*self[1][1] - self[0][1]*self[1][0]
    }
}

macro_rules! det {
    ($i:expr) => {
        impl<F> Det for [[F; $i]; $i]
        where
            F: Copy + Mul<F, Output = F> + Add<F, Output = F>,
            Self: SquareMatrix + Minor<Output = F, Index = (usize, usize)>
        {
            type Output = F;
        
            fn det(&self) -> Self::Output
            {
                (0..self.length())
                    .map(|i| self[i][0]*self.minor((i, 0)))
                    .reduce(|a, b| a + b)
                    .unwrap()
            }
        }
    };
}

det!(3);
det!(4);
det!(5);
det!(6);
det!(7);
det!(8);
det!(9);
det!(10);
det!(11);
det!(12);
det!(13);
det!(14);
det!(15);
det!(16);