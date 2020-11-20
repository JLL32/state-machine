struct BottleFillingMachine<S> {
    shared_value: usize,
    state: S,
}

struct Waiting {
    waiting_time: std::time::Duration,
}

struct Filling {
    rate: usize,
}

struct Done;

impl From<BottleFillingMachine<Waiting>> for BottleFillingMachine<Filling> {
    fn from(val: BottleFillingMachine<Waiting>) -> Self {
        BottleFillingMachine {
            shared_value: val.shared_value,
            state: Filling { rate: 1 },
        }
    }
}

impl From<BottleFillingMachine<Filling>> for BottleFillingMachine<Done> {
    fn from(val: BottleFillingMachine<Filling>) -> Self {
        BottleFillingMachine {
            shared_value: val.shared_value,
            state: Done {},
        }
    }
}

impl BottleFillingMachine<Waiting> {
    fn new(shared_value: usize) -> Self {
        BottleFillingMachine {
            shared_value,
            state: Waiting {
                waiting_time: std::time::Duration::new(0, 0),
            },
        }
    }
}

fn main() {
    let bottle_filler = BottleFillingMachine::new(0);
    
    // (Mock) Check on some shared and state-specific values
    assert_eq!(bottle_filler.state.waiting_time, std::time::Duration::new(0, 0));
    assert_eq!(bottle_filler.shared_value, 0);
    
    // Transition
    let bottle_filler = BottleFillingMachine::<Filling>::from(bottle_filler);
    
    // Can't do this anymore, it's been consumed!:
    // assert_eq!(bottle_filler.state.waiting_time, std::time::Duration::new(0, 0));
    
    let bottle_filler = BottleFillingMachine::<Done>::from(bottle_filler);
}
