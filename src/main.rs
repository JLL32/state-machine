// This is some functionality shared by all of the states.
trait SharedFunctionality {
    fn get_shared_value(&self) -> usize;
}

struct Waiting {
    waiting_time: std::time::Duration,
    // Value shared by all the states.
    shared_value: usize,
}
impl From<Done> for Waiting {
    fn from(val: Done) -> Self {
        Self {
            waiting_time: std::time::Duration::new(0, 0),
            shared_value: val.shared_value,
        }
    }
}
impl SharedFunctionality for Waiting {
    fn get_shared_value(&self) -> usize {
        self.shared_value
    }
}


struct Filling {
    rate: usize,
    // Value shared by all the states.
    shared_value: usize,
}
impl From<Waiting> for Filling {
    fn from(val: Waiting) -> Self {
        Self {
            rate: 1,
            shared_value: val.shared_value,
        }
    }
}
impl SharedFunctionality for Filling {
    fn get_shared_value(&self) -> usize {
        self.shared_value
    }
}


struct Done {
    shared_value: usize,
}
impl From<Filling> for Done {
    fn from(val: Filling) -> Self {
        Self {
            shared_value: val.shared_value,
        }
    }
}
impl SharedFunctionality for Done {
    fn get_shared_value(&self) -> usize {
        self.shared_value
    }
}

enum State {
    Waiting(Waiting),
    Filling(Filling),
    Done(Done),
}

fn main() {
}
