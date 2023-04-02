use rayon::iter::*;
use crate::matrix_transpose;

// dot product and matrix multiply
fn par_dot_product(x: &Vec<f64>, y: &Vec<f64>) -> f64 {
    x.par_iter().zip(y).map(|(&xk, &yk)| xk * yk).sum()
}

fn seq_dot_product(x: &Vec<f64>, y: &Vec<f64>) -> f64 {
    x.iter().zip(y).map(|(&xk, &yk)| xk * yk).sum()
}

fn matrix_mult(m1: Vec<Vec<f64>>, m2: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut ans = vec![vec![0f64; m2[0].len()]; m1.len()];
    let m2t: Vec<Vec<f64>> = matrix_transpose::par_transpose(m2.clone());

    // currently sequential (make faster)
    // check size (m1 row and m2 col need to be match)
    (0..ans.len())
        .for_each(|i|(0..ans[0].len())
            .for_each(|j| ans[i][j] = par_dot_product(&m1[i], &m2t[j])));
    ans
}

#[cfg(test)]
mod tests {
    use std::iter::zip;
    use super::{par_dot_product, seq_dot_product, matrix_mult};
    use std::time::{Duration, Instant};

    fn timed<R, F>(f: F) -> (R, Duration) where F: Fn() -> R {
        let starting_point = Instant::now();
        let res = f();
        (res, starting_point.elapsed())
    }

    #[test]
    fn dot_test() {
        // TODO: implement test cases
        assert_eq!(0.0, par_dot_product(&vec![], &vec![]));
        assert_eq!(0.0, par_dot_product(&vec![0.0], &vec![0.0]));
        assert_eq!(14.0, par_dot_product(&vec![1.0, 2.0, 3.0], &vec![1.0, 2.0, 3.0]));

        assert_eq!(0.0, seq_dot_product(&vec![], &vec![]));
        assert_eq!(0.0, seq_dot_product(&vec![0.0], &vec![0.0]));
        assert_eq!(14.0, seq_dot_product(&vec![1.0, 2.0, 3.0], &vec![1.0, 2.0, 3.0]));

        let a_seq = (0..=1024).map(|a| a as f64).collect::<Vec<_>>();
        let (output, time) = timed(|| seq_dot_product(&a_seq, &a_seq));
        println!("sequential with 1024-size output: {:?} time: {:?}", output, time);

        let a = (0..=1024).map(|a| a as f64).collect::<Vec<_>>();
        let (output, time) = timed(|| par_dot_product(&a, &a));
        println!("parallel with 1024-size output: {:?} time: {:?}", output, time);

        let b_seq = (0..=20_000_000).map(|a| a as f64).collect::<Vec<_>>();
        let (output, time) = timed(|| seq_dot_product(&b_seq, &b_seq));
        println!("sequential with 20_000_000-size output: {:?} time: {:?}", output, time);

        let b = (0..=20_000_000).map(|a| a as f64).collect::<Vec<_>>();
        let (output, time) = timed(|| par_dot_product(&b, &b));
        println!("parallel with 20_000_000-size output: {:?} time: {:?}", output, time);
    }

    #[test]
    fn mult_test() {
        // TODO: implement test cases -> error when size not match
        println!("{:?}", matrix_mult(vec![vec![1.0, 2.0, 3.0],
                                                vec![4.0, 5.0, 6.0],
                                                vec![7.0, 8.0, 9.0],
                                                vec![10.0, 11.0, 12.0]],
                                     vec![vec![1.0, 2.0, 3.0],
                                                vec![4.0, 5.0, 6.0],
                                                vec![7.0, 8.0, 9.0]]));
    }
}
