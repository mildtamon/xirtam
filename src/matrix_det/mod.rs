use rayon::prelude::*;
use crate::matrix_multiply_dot::{par_dot_product, par_matrix_mult, seq_matrix_mult};
use crate::matrix_transpose;

fn par_det(m: Vec<Vec<f64>>) -> f64 {
    let (q, r) = par_qr_decomposition(m);
    r.par_iter()
        .enumerate()
        .map(|(i, row)| row[i])
        .product()
}

fn seq_det(m: Vec<Vec<f64>>) -> f64 {
    let (q, r) = seq_qr_decomposition(m);
    r.par_iter()
        .enumerate()
        .map(|(i, row)| row[i])
        .product()
}

// euclidean norm
fn par_norm(v: Vec<f64>) -> Vec<f64> {
    let norm: f64 = v.par_iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
    v.iter().map(|each| each / norm).collect()
}

// projection of vector v to u
fn par_project(v: &Vec<f64>, u: &Vec<f64>) -> Vec<f64> {
    let norm_u = u.par_iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
    u.par_iter().map(|each| each *( par_dot_product(v, u) / norm_u)).collect()
}

fn gram_schmidt(m: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut q: Vec<Vec<f64>> = vec![];
    for j in 0..m[0].len() {
        let mut v: Vec<f64> = m.par_iter().map(|row| row[j]).collect();
        for i in 0..j {
            let u = &q[i];
            let proj = par_project(&v, u);
            for k in 0..v.len() {
                v[k] -= proj[k];
            }
        }
        q.push(par_norm(v));
    }
    q
}

fn par_qr_decomposition(m: Vec<Vec<f64>>) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
    let q = matrix_transpose::par_transpose(gram_schmidt(&m));
    let r = par_matrix_mult(matrix_transpose::par_transpose(q.clone()), m);
    (q, r)
}

fn seq_qr_decomposition(m: Vec<Vec<f64>>) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
    let q = matrix_transpose::seq_transpose(gram_schmidt(&m));
    let r = seq_matrix_mult(matrix_transpose::seq_transpose(q.clone()), m);
    (q, r)
}

#[cfg(test)]
mod tests {
    use super::{par_det, seq_det};
    use std::time::{Duration, Instant};

    fn timed<R, F>(f: F) -> (R, Duration) where F: Fn() -> R {
        let starting_point = Instant::now();
        let res = f();
        (res, starting_point.elapsed())
    }

    #[test]
    fn qr_test() {
        let matrix = vec![
            vec![1.0, 0.0, -2.0],
            vec![3.0, 1.0, -2.0],
            vec![-5.0, -1.0, 9.0], ];

        assert_eq!(3.0, par_det(matrix).round());

        let two_d_matrix = vec![(0..=400).map(|a| a as f64).collect::<Vec<_>>(); 400];

        let (output, time) = timed(|| par_det(two_d_matrix.clone()));
        println!("parallel determinant of matrix with 400-size    time: {:?}", time);

        let (output, time) = timed(|| seq_det(two_d_matrix.clone()));
        println!("sequential determinant of matrix with 400-size    time: {:?}", time);
    }
}
