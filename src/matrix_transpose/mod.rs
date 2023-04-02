use rayon::iter::*;

pub fn seq_transpose(m: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    // TODO: implement sequential matrix transpose
    (0..m[0].len()).into_iter()
        .map(|i| m.iter().map(|row| row[i]).collect())
        .collect()
}

pub fn par_transpose(m: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    // TODO: implement parallel matrix transpose
    (0..m[0].len()).into_par_iter()
        .map(|i| m.par_iter().map(|row| row[i]).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{seq_transpose,par_transpose};

    #[test]
    fn transpose_test() {
        // TODO: implement test cases
        println!("{:?}", seq_transpose(vec![vec![1.0, 3.0, 5.0], vec![2.0, 4.0, 6.0]]));
        println!("{:?}", par_transpose(vec![vec![1.0, 3.0, 5.0], vec![2.0, 4.0, 6.0]]));
    }
}
