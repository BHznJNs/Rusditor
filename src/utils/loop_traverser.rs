pub struct LoopTraverser<T> {
    vec: Vec<T>,
    index: isize,
    cycle: bool,
}

impl<T> LoopTraverser<T> {
    pub fn new(cycle: bool) -> Self {
        Self {
            vec: Vec::<T>::new(),
            index: -1,
            cycle,
        }
    }
    #[inline]
    fn current<'a>(&'a self) -> &'a T {
        &self.vec[self.index as usize]
    }

    pub fn next<'a>(&'a mut self) -> Option<&'a T> {
        if self.vec.is_empty() || (!self.cycle && self.index == (self.vec.len() - 1) as isize) {
            return None;
        }

        self.index = (self.index + 1) % (self.vec.len() as isize);
        return Some(self.current());
    }
    pub fn previous<'a>(&'a mut self) -> Option<&'a T> {
        if self.vec.is_empty() || (!self.cycle && (self.index == 0 || self.index == -1)) {
            return None;
        }

        self.index = if self.index == 0 || self.index == -1 {
            (self.vec.len() - 1) as isize
        } else {
            (self.index - 1) % (self.vec.len() as isize)
        };
        return Some(self.current());
    }

    #[inline]
    pub fn push(&mut self, element: T) {
        self.vec.push(element);
    }

    #[inline]
    pub fn reset_index(&mut self) {
        self.index = -1;
    }

    #[inline]
    pub fn set_content(&mut self, new_vec: Vec<T>) {
        self.vec = new_vec;
        self.reset_index();
    }

    #[inline]
    pub fn clear(&mut self) {
        self.vec.clear();
        self.reset_index();
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }
}

#[test]
fn test() {
    let mut cycling_traverser = LoopTraverser::new(true);
    cycling_traverser.set_content(vec![1, 2, 3, 4]);

    assert_eq!(cycling_traverser.previous(), Some(&4));
    cycling_traverser.reset_index();
    assert_eq!(cycling_traverser.next(), Some(&1));
    assert_eq!(cycling_traverser.next(), Some(&2));
    assert_eq!(cycling_traverser.next(), Some(&3));
    assert_eq!(cycling_traverser.next(), Some(&4));

    // --- --- --- --- --- ---

    let mut uncycling_traverser = LoopTraverser::new(false);
    uncycling_traverser.set_content(vec![1, 2, 3, 4]);

    assert_eq!(uncycling_traverser.previous(), None);
    assert_eq!(uncycling_traverser.next(), Some(&1));
    assert_eq!(uncycling_traverser.next(), Some(&2));
    assert_eq!(uncycling_traverser.next(), Some(&3));
    assert_eq!(uncycling_traverser.next(), Some(&4));
    assert_eq!(uncycling_traverser.next(), None);
    assert_eq!(uncycling_traverser.previous(), Some(&3));
}