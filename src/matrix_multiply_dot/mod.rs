// dot product and matrix multiply

fn par_dot_product(x: &[f64], y: &[f64]) -> f64 {
    use rayon::iter::*;
    x.par_iter().zip(y).map(|(&xk, &yk)| xk * yk).sum()
}
