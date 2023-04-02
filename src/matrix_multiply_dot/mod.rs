use rayon::iter::*;
use crate::matrix_transpose;

fn par_dot_product(x: &Vec<f64>, y: &Vec<f64>) -> f64 {
    x.par_iter().zip(y).map(|(&xk, &yk)| xk * yk).sum()
}

fn seq_dot_product(x: &Vec<f64>, y: &Vec<f64>) -> f64 {
    x.iter().zip(y).map(|(&xk, &yk)| xk * yk).sum()
}

fn seq_matrix_mult(m1: Vec<Vec<f64>>, m2: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut ans = vec![vec![0f64; m2[0].len()]; m1.len()];
    let m2t: Vec<Vec<f64>> = matrix_transpose::par_transpose(m2.clone());
    (0..ans.len())
        .for_each(|i|(0..ans[0].len())
            .for_each(|j| ans[i][j] = par_dot_product(&m1[i], &m2t[j])));
    ans
}

fn par_matrix_mult(m1: Vec<Vec<f64>>, m2: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let m2t: Vec<Vec<f64>> = matrix_transpose::par_transpose(m2.clone());
    m1.par_iter()
        .map(|r| {
            (0..m2t.len())
                .into_par_iter()
                .map(|c| par_dot_product(r, &m2t[c]))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use std::iter::zip;
    use super::{par_dot_product, seq_dot_product, par_matrix_mult, seq_matrix_mult};
    use std::time::{Duration, Instant};

    fn timed<R, F>(f: F) -> (R, Duration) where F: Fn() -> R {
        let starting_point = Instant::now();
        let res = f();
        (res, starting_point.elapsed())
    }

    #[test]
    fn dot_test() {
        assert_eq!(0.0, seq_dot_product(&vec![], &vec![]));
        assert_eq!(0.0, seq_dot_product(&vec![0.0], &vec![0.0]));
        assert_eq!(14.0, seq_dot_product(&vec![1.0, 2.0, 3.0], &vec![1.0, 2.0, 3.0]));

        assert_eq!(0.0, par_dot_product(&vec![], &vec![]));
        assert_eq!(0.0, par_dot_product(&vec![0.0], &vec![0.0]));
        assert_eq!(14.0, par_dot_product(&vec![1.0, 2.0, 3.0], &vec![1.0, 2.0, 3.0]));

        let one_d_matrix = (0..=1024).map(|a| a as f64).collect::<Vec<_>>();
        let (output, time) = timed(|| seq_dot_product(&one_d_matrix, &one_d_matrix));
        println!("sequential dot product with 1024-size   output: {:?}  time: {:?}", output, time);

        let (output, time) = timed(|| par_dot_product(&one_d_matrix, &one_d_matrix));
        println!("parallel dot product with 1024-size     output: {:?}  time: {:?}", output, time);

        let (output, time) = timed(|| seq_dot_product(&one_d_matrix, &one_d_matrix));
        println!("sequential dot product with 20_000_000-size   output: {:?}    time: {:?}", output, time);

        let (output, time) = timed(|| par_dot_product(&one_d_matrix, &one_d_matrix));
        println!("parallel dot product with 20_000_000-size     output: {:?}    time: {:?}", output, time);
    }

    #[test]
    fn mult_test() {
        assert_eq!(vec![vec![30.0, 36.0, 42.0],
                      vec![66.0, 81.0, 96.0],
                      vec![102.0, 126.0, 150.0],
                      vec![138.0, 171.0, 204.0]],
                 seq_matrix_mult(vec![vec![1.0, 2.0, 3.0],
                                                vec![4.0, 5.0, 6.0],
                                                vec![7.0, 8.0, 9.0],
                                                vec![10.0, 11.0, 12.0]],
                                     vec![vec![1.0, 2.0, 3.0],
                                                vec![4.0, 5.0, 6.0],
                                                vec![7.0, 8.0, 9.0]]));
        assert_eq!(vec![vec![30.0, 36.0, 42.0],
                        vec![66.0, 81.0, 96.0],
                        vec![102.0, 126.0, 150.0],
                        vec![138.0, 171.0, 204.0]],
                   par_matrix_mult(vec![vec![1.0, 2.0, 3.0],
                                        vec![4.0, 5.0, 6.0],
                                        vec![7.0, 8.0, 9.0],
                                        vec![10.0, 11.0, 12.0]],
                                   vec![vec![1.0, 2.0, 3.0],
                                        vec![4.0, 5.0, 6.0],
                                        vec![7.0, 8.0, 9.0]]));

        let two_d_matrix = vec![(0..=1024).map(|a| a as f64).collect::<Vec<_>>(); 1024];

        let (output, time) = timed(|| seq_matrix_mult(two_d_matrix.clone(), two_d_matrix.clone()));
        println!("sequential matrix multiply with 1024-size   time: {:?}", time);
        // println!("sequential matrix multiply with 1024-size   time: {:?}    output: {:?}", time, output);

        let (output, time) = timed(|| par_matrix_mult(two_d_matrix.clone(), two_d_matrix.clone()));
        println!("parallel matrix multiply with 1024-size     time: {:?}", time);
        // println!("parallel matrix multiply with 1024-size     time: {:?}    output: {:?}", time, output);
    }

}
