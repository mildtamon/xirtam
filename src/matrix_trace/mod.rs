use rayon::iter::*;

fn seq_trace(m: &Vec<Vec<f64>>) -> (f64) {
    // sequential matrix trace is better
    (0..m.len()).into_iter().map(|index| m[index][index]).sum()
}

fn par_trace(m: &Vec<Vec<f64>>) -> (f64) {
    let mut sum = 0.0;
    (0..m.len()).into_iter().for_each(|index| sum += m[index][index]);
    sum
}

#[cfg(test)]
mod tests {
    use super::{seq_trace, par_trace};
    use std::time::{Duration, Instant};

    fn timed<R, F>(f: F) -> (R, Duration) where F: Fn() -> R {
        let starting_point = Instant::now();
        let res = f();
        (res, starting_point.elapsed())
    }

    #[test]
    fn trace_test() {
        // TODO: implement more test cases
        assert_eq!(15.0, seq_trace(&vec![vec![1.0 ,2.0 ,3.0],
                                        vec![4.0 ,5.0 ,6.0],
                                        vec![7.0 ,8.0 ,9.0]]));
        assert_eq!(0.0, seq_trace(&vec![]));
        assert_eq!(15.0, par_trace(&vec![vec![1.0 ,2.0 ,3.0],
                                     vec![4.0 ,5.0 ,6.0],
                                     vec![7.0 ,8.0 ,9.0]]));
        assert_eq!(0.0, par_trace(&vec![]));

        let two_d_matrix = vec![(0..=1024).map(|a| a as f64).collect::<Vec<_>>(); 1024];

        let (output, time) = timed(|| seq_trace(&two_d_matrix.clone()));
        println!("sequential matrix trace with 1024-size   time: {:?}  output: {:?}", time, output);

        let (output, time) = timed(|| par_trace(&two_d_matrix.clone()));
        println!("parallel matrix trace with 1024-size     time: {:?}  output: {:?}", time, output);
    }
}
