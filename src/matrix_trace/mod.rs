use rayon::iter::*;

fn seq_trace(m: &Vec<Vec<f64>>) -> (f64) {
    // sequential matrix trace is better
    let mut sum: f64 = 0.0;
    for index in 0..m.len() { sum += m[index][index] }
    sum
    // (0..m.len()).into_iter().map(|r| m[r][r]).sum()
}

fn par_trace(m: &Vec<Vec<f64>>) -> (f64) {
    (0..m.len()).into_par_iter().map(|r| m[r][r]).sum()
}

#[cfg(test)]
mod tests {
    use super::{seq_trace,par_trace};

    #[test]
    fn trace_test() {
        // TODO: implement more test cases
        assert_eq!(15.0, seq_trace(&vec![vec![1.0 ,2.0 ,3.0],
                                        vec![4.0 ,5.0 ,6.0],
                                        vec![7.0 ,8.0 ,9.0]]));
        assert_eq!(0.0, trace(&vec![]));
        assert_eq!(15.0, par_trace(&vec![vec![1.0 ,2.0 ,3.0],
                                     vec![4.0 ,5.0 ,6.0],
                                     vec![7.0 ,8.0 ,9.0]]));
        assert_eq!(0.0, par_trace(&vec![]));
    }
}