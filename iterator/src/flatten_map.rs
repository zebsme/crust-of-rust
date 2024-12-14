
pub struct FlattenMap<O, F, U>
where
    O: IntoIterator,
    F: FnMut(O::Item) -> U,
    U: Iterator,
{
    outer: O::IntoIter,
    mapper: F,
    front_inner: Option<U>,
    back_inner: Option<U>,
}

pub fn flatten_map<O, F, U>(outer: O, mapper: F) -> FlattenMap<O::IntoIter, F, U>
where
    O: IntoIterator,
    F: FnMut(O::Item) -> U,
    U: Iterator,
{
    FlattenMap::new(outer.into_iter(), mapper)
}

impl<O, F, U> FlattenMap<O, F, U>
where
    O: Iterator,
    F: FnMut(O::Item) -> U,
    U: Iterator,
{
    pub fn new(outer: O, mapper: F) -> Self {
        FlattenMap {
            outer: outer,
            mapper,
            front_inner: None,
            back_inner: None,
        }
    }
}

impl<O, F, U> Iterator for FlattenMap<O, F, U>
where
    O: IntoIterator,
    F: FnMut(O::Item) -> U,
    U: Iterator,
{
    type Item = U::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner) = self.front_inner {
                if let Some(item) = inner.next() {
                    return Some(item);
                }
                self.front_inner = None;
            }

            if let Some(outer_item) = self.outer.next() {
                self.front_inner = Some((self.mapper)(outer_item));
            } else {
                return None;
            }
        }
    }
}

impl<O, F, U> DoubleEndedIterator for FlattenMap<O, F, U>
where
    O: DoubleEndedIterator,
    F: FnMut(O::Item) -> U,
    U: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner) = self.back_inner {
                if let Some(item) = inner.next_back() {
                    return Some(item);
                }
                self.back_inner = None;
            }

            if let Some(outer_item) = self.outer.next_back() {
                self.back_inner = Some((self.mapper)(outer_item));
            } else {
                return None;
            }
        }
    }
}
