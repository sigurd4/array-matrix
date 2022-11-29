#![feature(generic_const_exprs)]
#![recursion_limit = "256"]
#[allow(incomplete_features)]
#[allow(const_evaluatable_unchecked)]

pub mod matrix;
pub use crate::matrix::*;

#[cfg(test)]
mod tests {
    use crate::{Det, MAdd, Trace, Matrix, MInv, Cross, MMul};

    #[test]
    fn test_det()
    {
        let a = [
            [1, 2, 3]
        ];

        let a = [1.0, 0.0, 0.0];
        let b = [0.0, 1.0, 0.0];
        let ab = a.cross(b);
        println!("{}\n", ab.map(|abi| abi.to_string()).join(", "));

        let a = [
            [1.0],
            [2.0],
            [3.0]
        ];
        let b = [
            [1.0, 2.0, 3.0]
        ];
        let ab = a.mul(b);
        println!("{}\n", ab.map(|abr| abr.map(|abrc: f64| abrc.to_string()).join(", ")).join("\n"));

        let a: [[f32; 3]; 3] = [
            [0.5, -0.1, 0.3],
            [0.7, -0.2, 0.1],
            [3.0, -5.0, 1.0]
        ];
        println!("{}", a.det());
        match a.inv()
        {
            Some(i) => println!("{}", i.det()),
            None => ()
        };
    }

    #[test]
    fn system()
    {
        let a: [[f32; 2]; 2] = [
            [-0.5, -0.1],
            [10.0, -0.2]
        ];
        let b: [[f32; 1]; 2] = [
            [1.0],
            [0.0]
        ];
        let c: [[f32; 2]; 1] = [
            [0.0, 1.0]
        ];
        let d: f32 = 0.0;

        const N: usize = 1000;
        const T: f32 = 40.0;

        let mut x: [[f32; 1]; 2] = Matrix::empty();
        let mut y: [f32; N] = [0.0; N];
        let mut u: f32 = 0.0;

        let dt = T/N as f32;

        for i in 0..N
        {
            let dxdt = a.mul(x).add(b.mul(u));

            y[i] = c.mul(x).trace() + d*u;
            x = x.add(dxdt.mul(dt));

            u = 1.0;
        }

        println!("{}", y.map(|yn| yn.to_string()).join(", "))
    }
}
