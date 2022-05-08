use custom_logger::env_logger_init;

trait StateMachine<P> {
    //fn cur_state(&mut self) -> Box<&mut (dyn State<Protocol1> + 'static>);
    fn dispatch(&mut self, msg: &P);
}

pub trait State<P> {
    fn enter(&mut self) {
        log::debug!("State: enter");
    }
    fn process(&mut self, _msg: &P) {
        log::debug!("State: process:");
    }
    fn exit(&mut self) {
        log::debug!("State: exit");
    }
}

#[derive(Debug)]
enum Protocol1 {
    Msg1 { f1: i32 },
}

struct MySm {
    cur_state: Box<dyn State<Protocol1> + 'static>,
}

impl MySm {
    fn cur_state(&mut self) -> &mut Box<dyn State<Protocol1> + 'static> {
        log::debug!("MySm::cur_state():+-");
        &mut self.cur_state
    }
}

impl StateMachine<Protocol1> for MySm {
    //fn cur_state(&mut self) -> &Box<dyn State<Protocol1> + 'static> {
    //    &self.cur_state
    //}

    fn dispatch(&mut self, msg: &Protocol1) {
        log::debug!("MySm: process+");
        self.cur_state.process(msg); // Use cur_state directly
        self.cur_state().process(msg); // Use cur_state() to get cur_state indirectly
        log::debug!("MySm: process-");
    }
}

struct State1;

impl State<Protocol1> for State1 {
    fn process(&mut self, msg: &Protocol1) {
        match msg {
            Protocol1::Msg1 { f1 } => {
                log::debug!("State1: process m.f1={}", f1);
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

    let state1 = State1 {};
    let state2 = State2 {};

    let mut mysm = MySm {
        cur_state: Box::new(state1),
    };

    let msg = Protocol1::Msg1 { f1: 123 };
    mysm.cur_state.enter();
    mysm.dispatch(&msg);
    mysm.cur_state.exit();

    mysm.cur_state = Box::new(state2);
    mysm.cur_state.enter();
    let msg2 = Protocol1::Msg1 { f1: 456 };
    mysm.dispatch(&msg2);
    mysm.cur_state.exit();
}
