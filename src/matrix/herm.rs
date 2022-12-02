use num_complex::{Complex};
use num_traits::Float;

use crate::{matrix_init, Matrix};

pub trait Herm: Matrix
where
    Self::Output: Matrix
{
    type Output;
    fn herm(&self) -> Self::Output;
}

impl<F: Float, const L: usize, const H: usize> Herm for [[Complex<F>; L]; H]
where
    Self: Matrix,
    [[Complex<F>; H]; L]: Matrix
{
    type Output = [[Complex<F>; H]; L];
    fn herm(&self) -> Self::Output
    {
        matrix_init(|r, c| self[c][r].conj())
    }
}