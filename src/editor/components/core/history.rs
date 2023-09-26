use crate::utils::LoopTraverser;

pub struct ComponentHistory {
    list: LoopTraverser<String>,
}

impl ComponentHistory {
    pub fn new() -> Self {
        Self {
            list: LoopTraverser::new(false),
        }
    }

    #[inline]
    pub fn next<'a>(&'a mut self) -> Option<&'a String> {
        self.list.previous()
    }
    #[inline]
    pub fn previous<'a>(&'a mut self) -> Option<&'a String> {
        self.list.next()
    }

    #[inline]
    pub fn last<'a>(&'a self) -> Option<&'a String> {
        self.list.first()
    }

    #[inline]
    pub fn append(&mut self, element: String) {
        self.list.push_front(element);
    }
    #[inline]
    pub fn reset_index(&mut self) {
        self.list.reset_index();
    }
}
