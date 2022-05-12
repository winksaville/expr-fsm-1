use custom_logger::env_logger_init;

// Trait for Finite State Machines
pub trait StateMachine<'a, P: 'a> where Self: Sized {
    /// Return the current state
    fn cur_state(&mut self) -> &dyn State<Self, P>;

    fn dispatch(&mut self, msg: &P) {
        let cs = self.cur_state();
        cs.process(self, msg);
    }
}

// Trait for States
pub trait State<SM, P> {
    /// Invoked on every message
    fn process(&self, sm: &mut SM,  msg: &P) {
        let _ = msg; // Keep the signature clean, pretend to use it here
        let _ = sm; // Keep the signature clean, pretend to use it here
        log::debug!("State: process");
    }
}

// Create a Protocal with two messages
#[derive(Debug)]
enum Protocol1 {
    Add { f1: i32 },
    Sub { f1: i32 },
}

struct MySm<'a> {
    current_state: &'a dyn State<Self, Protocol1>,
    data: i32,
}

impl<'a> MySm<'a> {
    fn new(initial_state: &'a dyn State<Self, Protocol1>) -> Self {
        MySm {
            current_state: initial_state,
            data: 0,
        }
    }
}

//impl<'a> StateMachine<Protocol1> for MySm<'a> {
impl<'a> StateMachine<'a, Protocol1> for MySm<'a> {
    fn cur_state(&mut self) -> &dyn State<Self, Protocol1> {
        log::debug!("MySm::StateMachine::cur_state():+-");
        self.current_state
    }
}

struct State1;

impl State1 {
    fn new() -> Self {
        State1
    }
}

impl<'a> State<MySm<'a>, Protocol1> for State1 {
    fn process(&self, sm: &mut MySm, msg: &Protocol1) {
        log::debug!("State1: process msg={:?}", msg);
        match msg {
            Protocol1::Add { f1 } => {
                sm.data += f1;
            }
            Protocol1::Sub { f1 } => {
                sm.data -= f1;
            }
        }
    }
}

fn main() {
    env_logger_init("info");
    log::info!("Hello, world!");

    let msg = Protocol1::Add { f1: 123 };
    let state1 = State1::new();
    let mut sm = MySm::new(&state1);
    sm.dispatch(&msg);
}
