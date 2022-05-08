use custom_logger::env_logger_init;

trait StateMachine<P> {
    fn cur_state(&mut self) -> &mut Box<dyn State<P> + 'static>;

    fn dispatch(&mut self, msg: &Box<P>) {
        log::debug!("StateMachine<P>::dispatch:+");
        self.cur_state().process(msg);
        log::debug!("StateMachine<P>::dispatch:-");
    }
}

#[allow(unused)]
pub trait State<P> {
    fn enter(&mut self, msg: &Box<P>) {
        log::debug!("State: enter");
    }
    fn process(&mut self, msg: &Box<P>) {
        log::debug!("State: process:");
    }
    fn exit(&mut self, msg: &Box<P>) {
        log::debug!("State: exit");
    }
}

#[derive(Debug)]
enum Protocol1 {
    Msg1 { f1: i32 },
    Msg2 { f1: &'static str },
}

struct MySm {
    cur_state: Box<dyn State<Protocol1> + 'static>,
}

impl StateMachine<Protocol1> for MySm {
    fn cur_state(&mut self) -> &mut Box<dyn State<Protocol1> + 'static> {
        log::debug!("MySm::StateMachine::cur_state():+-");
        &mut self.cur_state
    }
}

struct State1;

impl State<Protocol1> for State1 {
    fn process(&mut self, msg: &Box<Protocol1>) {
        match **msg {
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
    fn process(&mut self, msg: &Box<Protocol1>) {
        log::debug!("State2: process msg={:?}", msg);
    }
}

fn main() {
    env_logger_init("info");
    log::info!("Hello, world!");

    let state1 = State1 {};
    let state2 = State2 {};

    let mut mysm = MySm {
        cur_state: Box::new(state1),
    };

    let msg = Box::new(Protocol1::Msg1 { f1: 123 });
    mysm.cur_state.enter(&msg);
    mysm.dispatch(&msg);
    mysm.cur_state.exit(&msg);

    mysm.cur_state = Box::new(state2);
    let msg2 = Box::new(Protocol1::Msg2 { f1: "a string" });
    mysm.cur_state.enter(&msg2);
    mysm.dispatch(&msg2);
    mysm.cur_state.exit(&msg2);
}
