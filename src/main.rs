use custom_logger::env_logger_init;

pub trait State {
    fn enter(&mut self) {
        log::debug!("State: enter");
    }
    fn process(&mut self) {
        log::debug!("State: process");
    }
    fn exit(&mut self) {
        log::debug!("State: exit");
    }
}

struct MySm {
    cur_state: Box<dyn State + 'static>,
}

struct State1;

impl State for State1 {
    fn process(&mut self) {
        //, msg: ProtocolType) {
        log::debug!("State1: process");
    }
}

fn main() {
    env_logger_init("info");
    log::info!("Hello, world!");

    let state1 = State1 {};
    let mut mysm = MySm {
        cur_state: Box::new(state1),
    };

    mysm.cur_state.enter();
    mysm.cur_state.process();
    mysm.cur_state.exit();
}
