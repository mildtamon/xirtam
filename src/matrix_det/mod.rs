use std::ops::Index;
use rayon::prelude::*;
use crate::matrix_multiply_dot::{par_dot_product, par_matrix_mult};
use crate::matrix_transpose;
use crate::matrix_transpose::par_transpose;

// determinant
// input matrix -> REF matrix -> det(REF matrix)
fn det(m: Vec<Vec<f64>>) -> f64 {
    // TODO: implement parallel matrix determinant using QR decomposition
    let r:f64 = qr_decomposition(&m).1.par_iter()
        .enumerate()
        .map(|(i, row)| row[i])
        .product();
    r
}

fn forward_elim(m: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    panic!("not implemented yet!");
}

fn normalize(v: &Vec<f64>) -> Vec<f64> {
    let norm: f64 = v.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
    v.iter().map(|x| x / norm).collect()
}

fn project(v: &Vec<f64>, u: &Vec<f64>) -> Vec<f64> {
    let scalar = par_dot_product(v, u) / par_dot_product(u, u);
    u.iter().map(|x| x * scalar).collect()
}

fn gram_schmidt(a: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut q: Vec<Vec<f64>> = vec![];
    for j in 0..a[0].len() {
        let mut v: Vec<f64> = a.iter().map(|row| row[j]).collect();
        for i in 0..j {
            let u = &q[i];
            let proj = project(&v, u);
            for k in 0..v.len() {
                v[k] -= proj[k];
            }
        }
        q.push(normalize(&v));
    }
    q
}

fn qr_decomposition(a: Vec<Vec<f64>>) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
    let q = transpose(&gram_schmidt(&a));
    let r = par_matrix_mult(transpose(&q), a);
    (q, r)
}


#[cfg(test)]
mod tests {
    use crate::matrix_det::{forward_elima, qr_decomposition};
    use super::{det, forward_elim};

    #[test]
    fn det_test() {
        // TODO: implement test cases
        let matrix = vec![
            vec![2.0, 1.0, -1.0],
            vec![-3.0, -1.0, 2.0],
            vec![-2.0, 1.0, 2.0],
        ];
        let result = vec![
            vec![2.0, 1.0, -1.0],
            vec![0.0, 0.5, 0.5],
            vec![0.0, 0.0, -1.0],
        ];
        assert_eq!(forward_elim(&matrix),result)
    }
    #[test]
    fn tas() {
        let mut a: Vec<Vec<f64>> = vec![
            vec![1., 1., 1.],
            vec![2., 3., 1.],
            vec![-3., 4., -2.]
        ];
        println!("{:?}", forward_elima(&mut a))
    }

    #[test]
    fn qr_test() {
        let matrix = vec![
            vec![2.0, 1.0, -1.0],
            vec![-3.0, -1.0, 2.0],
            vec![-2.0, 1.0, 2.0],
        ];

        let matrix2 = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ];

        let matrix3 = vec![
            vec![1.0, 0.0, -2.0],
            vec![3.0, 1.0, -2.0],
            vec![-5.0, -1.0, 9.0],
        ];

        // let (q,r) = qr_decomposition(&matrix);
        let (q,r) = qr_decomposition(&matrix3);
        let (q,r) = qr_decomposition(&matrix);

        println!("{:?}", q);
        println!("{:?}", r);
        let ans = det(matrix);
        println!("{:?}",ans);
    }
}
