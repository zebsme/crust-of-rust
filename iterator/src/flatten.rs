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
// pub struct Flatten<O, U>
// where
//     O: Iterator,
//     U: Iterator,
// {
//     outer: O,
//     front_inner: Option<U>,
//     back_inner: Option<U>,
// }

// impl<O, U> Flatten<O, U>
// where
//     O: Iterator,
//     U: Iterator,
// {
//     pub fn new(outer: O) -> Self {
//         Flatten {
//             outer,
//             front_inner: None,
//             back_inner: None,
//         }
//     }
// }

// pub fn flatten<O, U>(outer: O) -> Flatten<O::IntoIter, U>
// where
//     O: IntoIterator,
//     O::Item: IntoIterator<IntoIter = U>,
//     U: Iterator,
// {
//     Flatten::new(outer.into_iter())
// }

// impl<O, U> Iterator for Flatten<O, U>
// where
//     O: Iterator,
//     O::Item: IntoIterator<IntoIter = U>,
//     U: Iterator,
// {
//     type Item = U::Item;

//     fn next(&mut self) -> Option<Self::Item> {
//         loop {
//             if let Some(ref mut inner) = self.front_inner {
//                 if let Some(item) = inner.next() {
//                     return Some(item);
//                 }
//                 self.front_inner = None; 
//             }

//             if let Some(outer_item) = self.outer.next() {
//                 self.front_inner = Some(outer_item.into_iter());
//             } else {
//                 return None;
//             }
//         }
//     }
// }

// impl<O, U> DoubleEndedIterator for Flatten<O, U>
// where
//     O: DoubleEndedIterator,
//     O::Item: IntoIterator<IntoIter = U>,
//     U: DoubleEndedIterator,
// {
//     fn next_back(&mut self) -> Option<Self::Item> {
//         loop {
//             if let Some(ref mut inner) = self.back_inner {
//                 if let Some(item) = inner.next_back() {
//                     return Some(item);
//                 }
//                 self.back_inner = None; 
//             }

//             if let Some(outer_item) = self.outer.next_back() {
//                 self.back_inner = Some(outer_item.into_iter());
//             } else {
//                 return None;
//             }
//         }
//     }
// }

