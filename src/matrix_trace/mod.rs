use rayon::iter::*;

fn trace(m: &Vec<Vec<f64>>) -> (f64) {
    // sequential matrix trace is better
    let mut sum: f64 = 0.0;
    for index in 0..m.len() { sum += m[index][index] }
    sum
}

#[cfg(test)]
mod tests {
    use super::{trace};

    #[test]
    fn trace_test() {
        // TODO: implement more test cases
        assert_eq!(15.0, trace(&vec![vec![1.0 ,2.0 ,3.0],
                                        vec![4.0 ,5.0 ,6.0],
                                        vec![7.0 ,8.0 ,9.0]]));
        assert_eq!(0.0, trace(&vec![]));
    }
}