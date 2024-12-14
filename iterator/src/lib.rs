pub struct Flatten<O>
where
    O: IntoIterator,
    O::Item: IntoIterator,
{
    outer: O::IntoIter,
    inner: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl <O> Flatten<O>
where
    O: IntoIterator,
    O::Item: IntoIterator,
{
    pub fn new(outer: O) -> Self {
        Flatten {
            outer: outer.into_iter(),
            inner: None,
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
            if let Some(ref mut inner_iter) = self.inner {
                if let Some(inner) = inner_iter.next() {
                    return Some(inner);
                }
                self.inner = None;
            }
            self.inner = Some(self.outer.next()?.into_iter());
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let data: Vec<Vec<()>> = vec![];
        assert_eq!(Flatten::new(data).count(), 0);
    }

    #[test]
    fn one_empty() {
        let data: Vec<Vec<()>> = vec![vec![]];
        assert_eq!(Flatten::new(data).count(), 0);
    }

    #[test]
    fn multi_empty() {
        let data: Vec<Vec<()>> = vec![vec![], vec![], vec![]];
        assert_eq!(Flatten::new(data).count(), 0);
    }

    #[test]
    fn one() {  
        let data = vec![vec![1]];
        assert_eq!(Flatten::new(data).count(), 1); 
    }

    #[test]
    fn multi_one() {  
        let data = vec![vec![1], vec![2], vec![3]];
        assert_eq!(Flatten::new(data).count(), 3); 
    }

    #[test]
    fn multi() {  
        let data = vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]];
        assert_eq!(Flatten::new(data).count(), 9); 
    }
}
