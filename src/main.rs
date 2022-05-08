use custom_logger::env_logger_init;

// Trait for Finite State Machines
trait StateMachine<P> {
    /// Return the current state
    fn cur_state(&mut self) -> &mut Box<dyn State<P> + 'static>;
    //fn cur_state(&mut self) -> &mut dyn State<P>;

    fn dispatch(&mut self, msg: &P) {
        log::debug!("StateMachine<P>::dispatch:+");
        self.cur_state().process(msg);
        log::debug!("StateMachine<P>::dispatch:-");
    }
}

// Trait for States
pub trait State<P> {
    /// Invoked when first transitioning to the state
    fn enter(&mut self, msg: &P) {
        let _ = msg; // Keep the signature clean, pretend to use it here
        log::debug!("State: enter");
    }

    /// Invoked on every message
    fn process(&mut self, msg: &P) {
        let _ = msg; // Keep the signature clean, pretend to use it here
        log::debug!("State: process");
    }

    /// Invoked when transitioning to a state, this includes if the transtion
    /// is to the current state, in which case exit then enter will be invoked.
    fn exit(&mut self, msg: &P) {
        let _ = msg; // Keep the signature clean, pretend to use it here
        log::debug!("State: exit");
    }
}

// Create a Protocal with two messages
#[derive(Debug)]
enum Protocol1 {
    Msg1 { f1: i32 },
    Msg2 { f1: &'static str },
}

struct MySm {
    current_state: Box<dyn State<Protocol1> + 'static>,
}

//impl<'a> StateMachine<Protocol1> for MySm<'a> {
impl StateMachine<Protocol1> for MySm {
    fn cur_state(&mut self) -> &mut Box<dyn State<Protocol1> + 'static> {
        log::debug!("MySm::StateMachine::cur_state():+-");
        &mut self.current_state
    }
}

struct State1;

impl State<Protocol1> for State1 {
    fn process(&mut self, msg: &Protocol1) {
        #[allow(unused)]
        match *msg {
            Protocol1::Msg1 { f1 } => {
                log::debug!("State1: process Msg1::f1={}", f1);
            }
            Protocol1::Msg2 { f1 } => {
                log::debug!("State1: process Msg2::f1={}", f1);
            }
        }
    }
}

struct State2;

impl State<Protocol1> for State2 {
    fn process(&mut self, msg: &Protocol1) {
        log::debug!("State2: process msg={:?}", msg);
    }
}

fn main() {
    env_logger_init("info");
    log::info!("Hello, world!");

    // Create state machine and initialize to State1
    let mut mysm = MySm {
        //current_state: Box::new(state1),
        current_state: Box::new(State1 {}),
    };

    // Allocate a message on the stack and dispatch it
    let msg = Protocol1::Msg1 { f1: 123 };
    mysm.current_state.enter(&msg);
    mysm.dispatch(&msg);
    mysm.current_state.exit(&msg);

    // Transition to State2
    mysm.current_state = Box::new(State2 {});

    // Allocate a second message on the heap and enter, dispatch msg then exit the state
    let msg2 = Protocol1::Msg2 { f1: "a string" };
    mysm.current_state.enter(&msg2);
    mysm.dispatch(&msg2);
    mysm.current_state.exit(&msg2);
}
