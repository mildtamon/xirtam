use std::ops::Index;
use rayon::prelude::*;

// determinant
// input matrix -> REF matrix -> det(REF matrix)
fn det(m: Vec<Vec<f64>>) -> f64 {
    // TODO: implement parallel matrix determinant using QR decomposition
    // forward_elim(&m).par_iter().enumerate().map(|(r,v)| v.index(r)).product()
    panic!("not implemented yet!");
}

fn forward_elim(m: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    panic!("not implemented yet!");
}



#[cfg(test)]
mod tests {
    use super::{det,forward_elim};

    #[test]
    fn det_test() {
        // TODO: implement test cases
        let matrix = vec![
            vec![2.0, 1.0, -1.0],
            vec![-3.0, -1.0, 2.0],
            vec![-2.0, 1.0, 2.0],
        ];
        let result = vec![
            vec![2.0, 1.0, -1.0],
            vec![0.0, 0.5, 0.5],
            vec![0.0, 0.0, -1.0],
        ];
        assert_eq!(forward_elim(&matrix),result)
    }
}
