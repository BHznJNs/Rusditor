use std::collections::VecDeque;

use super::event::EditorEvent;

pub struct EditorHistory {
    ops: VecDeque<EditorEvent>,

    // operations that is undone
    undo_ops: Vec<EditorEvent>,
    // operations that is redone
    redo_ops: Vec<EditorEvent>,
}

impl EditorHistory {
    const MAX_CACHED_EVENT: usize = 255;

    pub fn new() -> Self {
        Self {
            ops: VecDeque::<EditorEvent>::new(),
            undo_ops: Vec::<EditorEvent>::new(),
            redo_ops: Vec::<EditorEvent>::new(),
        }
    }

    pub fn undo<'a>(&'a mut self) -> Option<&'a EditorEvent> {
        let option_op = if self.ops.is_empty() {
            self.redo_ops.pop()
        } else {
            self.ops.pop_back()
        };

        match option_op {
            Some(op) => {
                self.undo_ops.push(op);
                self.undo_ops.last()
            }
            None => None,
        }
    }

    pub fn redo<'a>(&'a mut self) -> Option<&'a EditorEvent> {
        match self.undo_ops.pop() {
            Some(op) => {
                self.redo_ops.push(op);
                self.redo_ops.last()
            }
            None => None,
        }
    }

    pub fn append(&mut self, ev: EditorEvent) {
        self.undo_ops.clear();
        self.redo_ops.clear();
        self.ops.push_back(ev);

        if self.ops.len() > Self::MAX_CACHED_EVENT {
            self.ops.pop_front();
        }
    }
}
