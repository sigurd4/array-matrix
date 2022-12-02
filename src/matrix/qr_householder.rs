use num_complex::Complex;
use num_traits::{Float, Zero, One};

use crate::{Matrix, SquareMatrix, matrix_init, MMul, Transpose};

pub trait QRHouseholder: Matrix
{
    type OutputQ;
    type OutputR;

    /// Returns the Householder QR-decomposition of the given matrix
    /// 
    /// A = QR
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let a = [
    ///     [1.0, 2.0],
    ///     [3.0, 4.0]
    /// ];
    /// let (q, r) = a.qr_householder();
    /// ```
    fn qr_householder(&self) -> (Self::OutputQ, Self::OutputR);
}

impl<F: Float, const L: usize, const H: usize> QRHouseholder for [[Complex<F>; L]; H]
where
    Self: Matrix,
    [[Complex<F>; H]; H]: SquareMatrix
{
    type OutputQ = [[Complex<F>; H]; H];
    type OutputR = [[Complex<F>; L]; H];

    fn qr_householder(&self) -> (Self::OutputQ, Self::OutputR)
    {
        assert!(H >= L);
        let mut a: Vec<Vec<Complex<F>>> = (0..H)
            .map(|r| (0..L)
                .map(|c| self[r][c])
                .collect()
            ).collect();
        let mut q: [[Complex<F>; H]; H] = SquareMatrix::identity();
        for t in 0..L.min(H - 1)
        {
            let x: Vec<Complex<F>> = (0..H - t)
                .map(|r| a[r][0])
                .collect();
            let x_abs = x.iter()
                .map(|xn| xn.norm_sqr())
                .reduce(|a, b| a + b)
                .unwrap_or(F::zero())
                .sqrt();
            let alpha = -Complex::cis(x[t].arg())*x_abs;
            let mut u = x;
            u[0] = u[0] - alpha;
            let u_abs = u.iter()
                .map(|un| un.norm_sqr())
                .reduce(|a, b| a + b)
                .unwrap_or(F::zero())
                .sqrt();
            let v: Vec<Complex<F>> = u.iter()
                .map(|un| un/u_abs)
                .collect();
            let q_: Vec<Vec<Complex<F>>> = (0..H - t)
                .map(|r| (0..H - t)
                    .map(|c| if r == c {Complex::<F>::one()} else {Complex::zero()} - v[r]*v[c].conj()*F::from(2.0).unwrap())
                    .collect()
                ).collect();
            let qt: [[Complex<F>; H]; H] = matrix_init(|r, c| if r >= t && c >= t
                {
                    q_[c - t][r - t]
                }
                else
                {
                    if r == c {Complex::one()} else {Complex::zero()}
                }
            );
            q = q.mul(qt);
            a = (1..H - t)
                .map(|r| (1..L - t)
                    .map(|c| (0..H - t)
                        .map(|i| q_[r][i]*a[i][c])
                        .reduce(|a, b| a + b)
                        .unwrap_or(Complex::zero())
                    ).collect()
                ).collect()
        }
        let mut r = q.transpose().mul(self.clone());
        for i in 0..L
        {
            for j in i + 1..H
            {
                r[j][i] = Complex::zero();
            }
        }
        return (q, r)
    }
}

impl<const L: usize, const H: usize> QRHouseholder for [[f32; L]; H]
where
    Self: Matrix,
    [[Complex<f32>; L]; H]: QRHouseholder
{
    type OutputQ = <[[Complex<f32>; L]; H] as QRHouseholder>::OutputQ;
    type OutputR = <[[Complex<f32>; L]; H] as QRHouseholder>::OutputR;
    fn qr_householder(&self) -> (Self::OutputQ, Self::OutputR)
    {
        self.map(|ar| ar.map(|arc| Complex::from(arc))).qr_householder()
    }
}

impl<const L: usize, const H: usize> QRHouseholder for [[f64; L]; H]
where
    Self: Matrix,
    [[Complex<f64>; L]; H]: QRHouseholder
{
    type OutputQ = <[[Complex<f64>; L]; H] as QRHouseholder>::OutputQ;
    type OutputR = <[[Complex<f64>; L]; H] as QRHouseholder>::OutputR;
    fn qr_householder(&self) -> (Self::OutputQ, Self::OutputR)
    {
        self.map(|ar| ar.map(|arc| Complex::from(arc))).qr_householder()
    }
}