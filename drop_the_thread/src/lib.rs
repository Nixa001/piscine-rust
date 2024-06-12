use std::cell::{Cell, RefCell};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Workers {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>,
}

impl Workers {
    pub fn new() -> Workers {
        Workers {
            drops: Cell::new(0),
            states: RefCell::new(Vec::new()),
        }
    }

    pub fn new_worker(&self, cmd: String) -> (usize, Thread) {
        let pid = self.track_worker();
        let thread = Thread::new_thread(pid, cmd, self);
        (pid, thread)
    }

    pub fn track_worker(&self) -> usize {
        let index = self.states.borrow().len();
        self.states.borrow_mut().push(false);
        index
    }

    pub fn is_dropped(&self, id: usize) -> bool {
        *self.states.borrow().get(id).unwrap_or(&false)
    }

    pub fn add_drop(&self, id: usize) {
        let mut states = self.states.borrow_mut();
        if states[id] {
            panic!("{} is already dropped", id);
        }
        states[id] = true;
        self.drops.set(self.drops.get() + 1);
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Thread<'a> {
    pub pid: usize,
    pub cmd: String,
    pub parent: &'a Workers,
}

impl<'a> Thread<'a> {
    pub fn new_thread(pid: usize, cmd: String, parent: &'a Workers) -> Thread<'a> {
        Thread { pid, cmd, parent }
    }

    pub fn skill(self) {
        drop(self);
    }
}

impl<'a> Drop for Thread<'a> {
    fn drop(&mut self) {
        self.parent.add_drop(self.pid);
    }
}
