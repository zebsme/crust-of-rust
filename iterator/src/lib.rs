mod flatten;
mod flatten_map;

pub use flatten::flatten;
pub use flatten_map::flatten_map;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let data: Vec<Vec<()>> = vec![];
        assert_eq!(flatten(data.into_iter()).count(), 0);
    }

    #[test]
    fn one_empty() {
        let data: Vec<Vec<()>> = vec![vec![]];
        assert_eq!(flatten(data).count(), 0);
    }

    #[test]
    fn multi_empty() {
        let data: Vec<Vec<()>> = vec![vec![], vec![], vec![]];
        assert_eq!(flatten(data).count(), 0);
    }

    #[test]
    fn one() {
        let data = vec![vec![1]];
        assert_eq!(flatten(data).count(), 1);
    }

    #[test]
    fn multi_one() {
        let data = vec![vec![1], vec![2], vec![3]];
        assert_eq!(flatten(data).count(), 3);
    }

    #[test]
    fn multi() {
        let data = vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]];
        assert_eq!(flatten(data).count(), 9);
    }

    #[test]
    fn double_iterator() {
        let data = vec![vec!["a1", "a2", "a3"], vec!["b1", "b2", "b3"]];
        let mut iter = flatten(data);
        assert_eq!(iter.next(), Some("a1"));
        assert_eq!(iter.next_back(), Some("b3"));
        assert_eq!(iter.next(), Some("a2"));
        assert_eq!(iter.next_back(), Some("b2"));
        assert_eq!(iter.next(), Some("a3"));
        assert_eq!(iter.next_back(), Some("b1"));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn test_flatten_map() {
        let data = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let mut iter = flatten_map(data, |x| x.into_iter().map(|x| x * x));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next_back(), Some(36));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next_back(), Some(25));
        assert_eq!(iter.next(), Some(9));
        assert_eq!(iter.next_back(), Some(16));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }
}