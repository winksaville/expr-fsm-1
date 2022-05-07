use custom_logger::env_logger_init;

pub trait State<P> {
    fn enter(&mut self) {
        log::debug!("State: enter");
    }
    fn process(&mut self, _m: &P) {
        log::debug!("State: process:");
    }
    fn exit(&mut self) {
        log::debug!("State: exit");
    }
}

#[derive(Debug)]
enum Protocol1 {
    Msg1 {
        f1: i32,
    }
}

struct MySm {
    cur_state: Box<dyn State<Protocol1> + 'static>,
}

struct State1;

impl State<Protocol1> for State1 {
    fn process(&mut self, m: &Protocol1) {
        match m {
            Protocol1::Msg1 { f1 } => {
                log::debug!("State1: process m.f1={}", f1);
            }
        }
    }
}

fn main() {
    env_logger_init("info");
    log::info!("Hello, world!");

    let state1 = State1 {};
    let mut mysm = MySm {
        cur_state: Box::new(state1),
    };

    let msg = Protocol1::Msg1 {
        f1: 123,
    };
    mysm.cur_state.enter();
    mysm.cur_state.process(&msg);
    mysm.cur_state.exit();
}
