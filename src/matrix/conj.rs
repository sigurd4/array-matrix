use num_complex::ComplexFloat;

use crate::{matrix_init, Matrix};

pub trait Conj: Matrix
where
    Self::Output: Matrix
{
    type Output;
    fn conj(&self) -> Self::Output;
}

impl<F: ComplexFloat, const L: usize, const H: usize> Conj for [[F; L]; H]
where
    Self: Matrix
{
    type Output = Self;
    fn conj(&self) -> Self::Output
    {
        matrix_init(|r, c| self[r][c].conj())
    }
}