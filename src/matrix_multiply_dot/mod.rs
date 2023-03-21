use rayon::iter::*;

// dot product and matrix multiply
fn dot_product(x: &[f64], y: &[f64]) -> f64 {
    x.par_iter().zip(y).map(|(&xk, &yk)| xk * yk).sum()
}

fn matrix_mult(m1: Vec<Vec<i32>>, m2: Vec<Vec<i32>>) -> f64 {
    // TODO: implement parallel matrix multiply
    // idea: m1.iter(by row) m2.zip(by col) and use dot product on.

    panic!("not implemented yet!");
}

#[cfg(test)]
mod tests {
    use std::iter::zip;
    use super::{dot_product, matrix_mult};

    #[test]
    fn dot_test() {
        // TODO: implement test cases
        assert_eq!(0.0, dot_product(&vec![], &vec![]));
        assert_eq!(0.0, dot_product(&vec![0.0], &vec![0.0]));
        assert_eq!(14.0, dot_product(&vec![1.0, 2.0, 3.0], &vec![1.0, 2.0, 3.0]));
    }

    #[test]
    fn mult_test() {
        // TODO: implement test cases -> error when size not match
        let itr1 = vec![1, 2, 3, 4];
        let itr2 = vec![11, 12, 13 , 14];
        let itr3 = vec![21, 22, 23, 24];

    }
}
