use std::iter::FromIterator;
// A list is either nil -None- or an element with a list -Some(element, list)-
#[derive(Default)]
pub struct SimpleLinkedList<T>(Option<(T, Box<SimpleLinkedList<T>>)>);

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList(None)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    pub fn len(&self) -> usize {
        match &self.0 {
            None => 0,
            Some((_, next)) => next.len() + 1,
        }
    }

    pub fn push(&mut self, element: T) {
        *self = SimpleLinkedList(Some((element, Box::new(SimpleLinkedList(self.0.take())))))
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some((elem, list)) = self.0.take() {
            *self = *list;
            return Some(elem);
        }
        None
    }

    pub fn peek(&self) -> Option<&T> {
        if let Some((elem, _)) = &self.0 {
            return Some(elem);
        }
        None
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut newme = SimpleLinkedList::new();
        let mut tmp = self;
        while let Some((elem, list)) = tmp.0 {
            newme.push(elem);
            tmp = *list;
        }
        newme
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut r = SimpleLinkedList::new();
        for i in iter {
            r.push(i);
        }
        r
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut tmp = self;
        let mut vec = vec![];
        while let Some((elem, list)) = tmp.0 {
            vec.insert(0, elem);
            tmp = *list;
        }
        vec
    }
}
