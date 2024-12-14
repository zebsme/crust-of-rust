pub struct Flatten<O>
where
    O: IntoIterator,
    O::Item: IntoIterator,
{
    outer: O::IntoIter,
    front_inner: Option<<O::Item as IntoIterator>::IntoIter>,
    back_inner: Option<<O::Item as IntoIterator>::IntoIter>,
}

pub fn flatten<O>(outer: O) -> Flatten<O::IntoIter>
where
    O: IntoIterator,
    O::Item: IntoIterator,
{
    Flatten::new(outer.into_iter())
}

impl<O> Flatten<O>
where
    O: IntoIterator,
    O::Item: IntoIterator,
{
    pub fn new(outer: O) -> Self {
        Flatten {
            outer: outer.into_iter(),
            front_inner: None,
            back_inner: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: IntoIterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner_iter) = self.front_inner {
                if let Some(inner) = inner_iter.next() {
                    return Some(inner);
                }
                self.front_inner = None;
            }
            if let Some(item) = self.outer.next() {
                self.front_inner = Some(item.into_iter());
            } else{
                return self.back_inner.as_mut()?.next();
            }
        }
    }
}

impl<O> DoubleEndedIterator for Flatten<O>
where
    O: DoubleEndedIterator,
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner_iter) = self.back_inner {
                if let Some(inner) = inner_iter.next_back() {
                    return Some(inner);
                }
                self.back_inner = None;
            }
            if let Some(item) = self.outer.next_back() {
                self.back_inner = Some(item.into_iter());
            } else {
                return self.front_inner.as_mut()?.next_back();
            }
        }
    }
}


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
}
