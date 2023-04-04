use rayon::iter::*;

pub fn seq_transpose(m: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    (0..m[0].len()).into_iter()
        .map(|i| m.iter().map(|row| row[i]).collect())
        .collect()
}

pub fn par_transpose(m: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    (0..m[0].len()).into_par_iter()
        .map(|i| m.par_iter().map(|row| row[i]).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{seq_transpose,par_transpose};
    use std::time::{Duration, Instant};

    fn timed<R, F>(f: F) -> (R, Duration) where F: Fn() -> R {
        let starting_point = Instant::now();
        let res = f();
        (res, starting_point.elapsed())
    }

    #[test]
    fn transpose_test() {
        assert_eq!(vec![vec![1.0, 2.0],
                        vec![3.0, 4.0],
                        vec![5.0, 6.0]],
                   seq_transpose(vec![vec![1.0, 3.0, 5.0],
                                      vec![2.0, 4.0, 6.0]]));
        assert_eq!(vec![vec![1.0, 2.0],
                        vec![3.0, 4.0],
                        vec![5.0, 6.0]],
                   par_transpose(vec![vec![1.0, 3.0, 5.0],
                                      vec![2.0, 4.0, 6.0]]));

        let two_d_matrix = vec![(0..=1024).map(|a| a as f64).collect::<Vec<_>>(); 1024];

        let (output, time) = timed(|| seq_transpose(two_d_matrix.clone()));
        println!("sequential matrix transpose with 1024-size    time: {:?}", time);

        let (output, time) = timed(|| par_transpose(two_d_matrix.clone()));
        println!("parallel matrix transpose with 1024-size      time: {:?}", time);
    }
}
