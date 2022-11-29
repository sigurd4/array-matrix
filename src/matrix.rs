use num_traits::One;
use num_traits::Zero;

pub mod det;
pub mod submatrix;
pub mod inv;
pub mod transpose;
pub mod dot;
pub mod add;
pub mod trace;
pub mod adj;
pub mod minor;
pub mod cofactor;
pub mod cross;
pub mod kronecker;
pub mod mul;
pub mod outer;
pub mod conj;
pub mod herm;

pub use self::det::*;
pub use self::submatrix::*;
pub use self::inv::*;
pub use self::transpose::*;
pub use self::dot::*;
pub use self::add::*;
pub use self::trace::*;
pub use self::adj::*;
pub use self::minor::*;
pub use self::cofactor::*;
pub use self::cross::*;
pub use self::kronecker::*;
pub use self::mul::*;
pub use self::outer::*;
pub use self::conj::*;
pub use self::herm::*;

pub trait Matrix: Sized
{
    /// Returns the height of the given matrix
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// # #![feature(generic_const_exprs)]
    /// # #![allow(unused)]
    /// # extern crate matrix;
    /// # extern crate num_traits;
    /// # extern crate array_init;
    /// #
    /// let a = [
    ///     [1.0, 2.0, 3.0],
    ///     [4.0, 5.0, 6.0]
    /// ];
    /// assert_eq!(a.height(), 2);
    /// ```
    fn height(&self) -> usize;

    /// Returns the length of the given matrix
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// # #![feature(generic_const_exprs)]
    /// # #![allow(unused)]
    /// # extern crate matrix;
    /// # extern crate num_traits;
    /// # extern crate array_init;
    /// #
    /// let a = [
    ///     [1.0, 2.0, 3.0],
    ///     [4.0, 5.0, 6.0]
    /// ];
    /// assert_eq!(a.length(), 3);
    /// ```
    fn length(&self) -> usize;

    /// Returns an empty matrix with the given dimensions
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// # #![feature(generic_const_exprs)]
    /// # #![allow(unused)]
    /// # extern crate matrix;
    /// # extern crate num_traits;
    /// # extern crate array_init;
    /// #
    /// let a = [
    ///     [0.0, 0.0],
    ///     [0.0, 0.0]
    /// ]
    /// assert_eq!(a, Matrix::empty())
    /// ```
    fn empty() -> Self;
}

/// Initialize a matrix given an initializer expression.
/// The initializer is given the row- and collumn-indices of the cell.
/// It is allowed to mutate external state; we will always initialize the cells in order.
/// 
/// # Examples
/// 
/// ```rust
/// # #![feature(generic_const_exprs)]
/// # #![allow(unused)]
/// # extern crate matrix;
/// # extern crate num_traits;
/// # extern crate array_init;
/// #
/// let a: [[f64; 3]; 3] = matrix_init(|r, c| (r * c) as f64);
///
/// assert!(arr.iter().enumerate().all(|(r, ar)| ar.iter().enumerate().all(|(c, &arc)| arc == (r * c) as f64)));
/// ```
pub fn matrix_init<F, T, const L: usize, const H: usize>(mut initializer: F) -> [[T; L]; H]
where
    F: FnMut(usize, usize) -> T,
{
    use array_init::array_init;

    array_init(|r| array_init(|c| initializer(r, c)))
}

impl<F: Zero, const L: usize, const H: usize> Matrix for [[F; L]; H]
where [[F; L - 1 as usize]; H - 1]:
{
    fn height(&self) -> usize
    {
        H
    }
    fn length(&self) -> usize
    {
        L
    }
    fn empty() -> Self
    {
        matrix_init(|_, _| F::zero())
    }
}

pub trait SquareMatrix: Matrix
{
    /// Returns the identity matrix with the given dimensions
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// # #![feature(generic_const_exprs)]
    /// # #![allow(unused)]
    /// # extern crate matrix;
    /// # extern crate num_traits;
    /// # extern crate array_init;
    /// let a = [
    ///     [1.0, 0.0],
    ///     [0.0, 1.0]
    /// ]
    /// assert_eq!(a, SquareMatrix::identity())
    /// ```
    fn identity() -> Self;
}

impl<F: One + Zero, const N: usize> SquareMatrix for [[F; N]; N]
where 
    Self: Matrix,
    [[F; N - 1 as usize]; N - 1]:
{
    fn identity() -> Self
    {
        matrix_init(|r, c| if r == c {F::one()} else {F::zero()})
    }
}