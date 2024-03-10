fn first_n_smallest (arr: &[i32], n: usize) -> Vec<i32> {
    let mut indexes: Vec<(usize, &i32)> = arr.iter().enumerate().collect();
    indexes.sort_by(|a, b| (&a.1).cmp(&b.1));

    indexes = indexes[..n].to_vec();
    indexes.sort_by(|a, b| (&a.0).cmp(&b.0));

    indexes.iter().map(|x| *x.1).collect()
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(first_n_smallest(&[1,2,3,4,5],3), [1,2,3]);
        assert_eq!(first_n_smallest(&[5,4,3,2,1],3), [3,2,1]);
        assert_eq!(first_n_smallest(&[1,2,3,1,2],3), [1,2,1]);
        assert_eq!(first_n_smallest(&[1,2,3,-4,0],3), [1,-4,0]);
        assert_eq!(first_n_smallest(&[1,2,3,4,5],0), []);
        assert_eq!(first_n_smallest(&[1,2,3,4,5],5), [1,2,3,4,5]);
        assert_eq!(first_n_smallest(&[1,2,3,4,2],4), [1,2,3,2]);
        assert_eq!(first_n_smallest(&[2,1,2,3,4,2],2), [2,1]);
        assert_eq!(first_n_smallest(&[2,1,2,3,4,2],3), [2,1,2]);
        assert_eq!(first_n_smallest(&[2,1,2,3,4,2],4), [2,1,2,2]);
    }
}
