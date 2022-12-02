pub mod abs;
pub mod dot;
pub mod cross;
pub mod mul;
pub mod div;
pub mod outer;
pub mod add;
pub mod sub;
pub mod conj;

use num_traits::Zero;

pub use self::abs::*;
pub use self::dot::*;
pub use self::cross::*;
pub use self::mul::*;
pub use self::div::*;
pub use self::outer::*;
pub use self::add::*;
pub use self::sub::*;
pub use self::conj::*;

pub trait Vector
{
    fn length() -> usize;
    fn zero() -> Self;
}

impl<F: Zero, const N: usize> Vector for [F; N]
{
    fn length() -> usize
    {
        N
    }

    fn zero() -> Self
    {
        array_init::array_init(|_| F::zero())
    }
}