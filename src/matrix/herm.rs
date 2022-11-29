use num_complex::ComplexFloat;

use crate::{matrix_init, Matrix};

pub trait Herm: Matrix
where
    Self::Output: Matrix
{
    type Output;
    fn herm(&self) -> Self::Output;
}

impl<F: ComplexFloat, const L: usize, const H: usize> Herm for [[F; L]; H]
where
    Self: Matrix,
    [[F; H]; L]: Matrix
{
    type Output = [[F; H]; L];
    fn herm(&self) -> Self::Output
    {
        matrix_init(|r, c| self[c][r].conj())
    }
}