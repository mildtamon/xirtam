use rayon::iter::*;

// dot product and matrix multiply
fn dot_product(x: &[f64], y: &[f64]) -> f64 {
    x.par_iter().zip(y).map(|(&xk, &yk)| xk * yk).sum()
}

fn matrix_mult(m1: Vec<Vec<f64>>, m2: Vec<Vec<f64>>) -> f64 {
    // TODO: implement parallel matrix multiply
    // idea: m1.iter(by row) m2.zip(by col) and use dot product on.
    let ans = vec![vec![0f64; m2[0].len()]; m1.len()];

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
        println!("{:?}", matrix_mult(vec![vec![1.0, 2.0, 3.0],
                                                vec![4.0, 5.0, 6.0],
                                                vec![7.0, 8.0, 9.0]],
                                     vec![vec![1.0, 2.0, 3.0],
                                                vec![4.0, 5.0, 6.0],
                                                vec![7.0, 8.0, 9.0]]));
    }
}
