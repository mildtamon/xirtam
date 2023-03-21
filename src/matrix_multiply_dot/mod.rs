use rayon::iter::*;

// dot product and matrix multiply
fn par_dot_product(x: &[f64], y: &[f64]) -> f64 {
    x.par_iter().zip(y).map(|(&xk, &yk)| xk * yk).sum()
}

fn matrix_mult(m1: Vec<Vec<i32>>, m2: Vec<Vec<i32>>) -> f64 {
    // TODO: implement parallel matrix multiply
    panic!("not implemented yet!");
}

#[cfg(test)]
mod tests {
    use super::{par_dot_product, matrix_mult};

    #[test]
    fn dot_test() {
        // TODO: implement test cases -> check empty vector, different size
    }

    #[test]
    fn mult_test() {
        // TODO: implement test cases -> error when size not match
    }
}
