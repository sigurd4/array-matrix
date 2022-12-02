use num_complex::{Complex};
use num_traits::{Float};

use crate::{MMul, Diag, QRHouseholder, Matrix};

const ITERATIONS: usize = 1000;

pub trait Eig: Matrix
{
    type Output;

    /// Returns an approximation of the eigenvalues of a matrix
    /// 
    /// eig(A)
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let a = [
    ///     [1.0, 2.0],
    ///     [3.0, 4.0]
    /// ];
    /// let eig_a = eig(a);
    /// ```
    fn eig(&self) -> Self::Output;
}


impl<F: Float, const L: usize, const H: usize> Eig for [[Complex<F>; L]; H]
where
    [[Complex<F>; L]; H]: QRHouseholder,
    <[[Complex<F>; L]; H] as QRHouseholder>::OutputR: Diag
        + MMul<<[[Complex<F>; L]; H] as QRHouseholder>::OutputQ, Output = [[Complex<F>; L]; H]>
{
    type Output = <<[[Complex<F>; L]; H] as QRHouseholder>::OutputR as Diag>::Output;
    fn eig(&self) -> Self::Output
    {
        let mut a = self.clone();
        for _ in 0..ITERATIONS
        {
            let (q, r) = a.qr_householder();
            a = r.mul(q)
        }
        let r = a.qr_householder().1;
        r.diag()
    }
}

impl<const L: usize, const H: usize> Eig for [[f32; L]; H]
where
    Self: Matrix,
    [[Complex<f32>; L]; H]: Eig
{
    type Output = <[[Complex<f32>; L]; H] as Eig>::Output;
    fn eig(&self) -> Self::Output
    {
        self.map(|ar| ar.map(|arc| Complex::from(arc))).eig()
    }
}

impl<const L: usize, const H: usize> Eig for [[f64; L]; H]
where
    Self: Matrix,
    [[Complex<f64>; L]; H]: Eig
{
    type Output = <[[Complex<f64>; L]; H] as Eig>::Output;
    fn eig(&self) -> Self::Output
    {
        self.map(|ar| ar.map(|arc| Complex::from(arc))).eig()
    }
}