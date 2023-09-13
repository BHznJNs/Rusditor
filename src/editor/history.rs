use crate::utils::LoopTraverser;

pub struct History {
    list: LoopTraverser<String>
}

impl History {
    pub fn new() -> Self {
        Self {
            list: LoopTraverser::new(true),
        }
    }

    #[inline]
    pub fn next<'a>(&'a mut self) -> Option<&'a String> {
        self.list.next()
    }
    #[inline]
    pub fn previous<'a>(&'a mut self) -> Option<&'a String> {
        self.list.previous()
    }

    #[inline]
    pub fn append(&mut self, element: String) {
        self.list.push(element);
    }
    #[inline]
    pub fn reset_index(&mut self) {
        self.list.reset_index();
    }
}