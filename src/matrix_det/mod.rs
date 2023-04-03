use rayon::prelude::*;
use crate::matrix_multiply_dot::{par_dot_product, par_matrix_mult};
use crate::matrix_transpose;

fn det(m: Vec<Vec<f64>>) -> f64 {
    let (q, r) = qr_decomposition(m);
    r.par_iter()
        .enumerate()
        .map(|(i, row)| row[i])
        .product()
}

// euclidean norm
fn normalize(v: Vec<f64>) -> Vec<f64> {
    let norm: f64 = v.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
    v.iter().map(|each| each / norm).collect()
}

// projection of vector v to u
fn project(v: &Vec<f64>, u: &Vec<f64>) -> Vec<f64> {
    let norm_u = u.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
    u.par_iter().map(|each| each *( par_dot_product(v, u) / norm_u)).collect()
}

fn gram_schmidt(m: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut q: Vec<Vec<f64>> = vec![];
    for j in 0..m[0].len() {
        let mut v: Vec<f64> = m.par_iter().map(|row| row[j]).collect();
        for i in 0..j {
            let u = &q[i];
            let proj = project(&v, u);
            for k in 0..v.len() {
                v[k] -= proj[k];
            }
        }
        q.push(normalize(v));
    }
    q
}

fn qr_decomposition(m: Vec<Vec<f64>>) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
    let q = matrix_transpose::par_transpose(gram_schmidt(&m));
    let r = par_matrix_mult(matrix_transpose::par_transpose(q.clone()), m);
    (q, r)
}

#[cfg(test)]
mod tests {
    use super::{qr_decomposition, det};
    use std::time::{Duration, Instant};

    fn timed<R, F>(f: F) -> (R, Duration) where F: Fn() -> R {
        let starting_point = Instant::now();
        let res = f();
        (res, starting_point.elapsed())
    }

    #[test]
    fn qr_test() {
        let matrix = vec![
            vec![2.0, 1.0, -1.0],
            vec![-3.0, -1.0, 2.0],
            vec![-2.0, 1.0, 2.0], ];

        let matrix2 = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0], ];

        let matrix3 = vec![
            vec![1.0, 0.0, -2.0],
            vec![3.0, 1.0, -2.0],
            vec![-5.0, -1.0, 9.0], ];

        let (q,r) = qr_decomposition(matrix3.clone());
        println!("{:?}", q);
        println!("{:?}", r);
        let ans = det(matrix3.clone());
        println!("{:?}",ans);

        let two_d_matrix = vec![(0..=400).map(|a| a as f64).collect::<Vec<_>>(); 400];

        let (output, time) = timed(|| det(two_d_matrix.clone()));
        println!("determinant of matrix with 1024-size    time: {:?}", time);
    }
}
