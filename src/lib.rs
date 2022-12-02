#![feature(generic_const_exprs)]
#![feature(portable_simd)]
#![recursion_limit = "256"]
#[allow(incomplete_features)]
#[allow(const_evaluatable_unchecked)]

pub mod matrix;
pub mod vector;
pub use crate::matrix::*;
pub use crate::vector::*;

#[cfg(test)]
mod tests {
    use crate::{Det, MAdd, Trace, Matrix, MInv, Cross, MMul, Eig, QRHouseholder};

    #[test]
    fn test_det()
    {
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

    #[test]
    fn eig()
    {
        let a: [[f32; 2]; 2] = [
            [1.0, -0.8],
            [-3.0, 0.5]
        ];
        let eigs: Vec<String> = a.eig().iter().map(|yn| yn.to_string()).collect();
        println!("detA = {}", a.det());
        println!("lambda = {}", eigs.join(", "));
        println!("mul lambda = {}", a.eig().iter().map(|l| *l).reduce(|a, b| a*b).unwrap());
    }

    #[test]
    fn qr()
    {
        let a: [[f32; 2]; 2] = [
            [1.0, -0.8],
            [-3.0, 0.5]
        ];
        println!("a = [\n{}\n]", a.map(|ar| ar.map(|arc| arc.to_string()).join(", ")).join("\n"));
        let (q, r) = a.qr_householder();
        println!("q = [\n{}\n]", q.map(|ar| ar.map(|arc| arc.to_string()).join(", ")).join("\n"));
        println!("r = [\n{}\n]", r.map(|ar| ar.map(|arc| arc.to_string()).join(", ")).join("\n"));
        let qr = q.mul(r);
        println!("qr = [\n{}\n]", qr.map(|ar| ar.map(|arc| arc.to_string()).join(", ")).join("\n"));
    }
}
