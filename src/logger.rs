//#[feature(thread_id_value)]

use std::io::Write;

pub fn env_logger_init(default_level: &str) {
    let env = env_logger::Env::default();
    env_logger::Builder::from_env(env.default_filter_or(default_level))
        .format(|buf, record| {
            let time = std::time::SystemTime::now();
            writeln!(
                buf,
                "[{} {:5} {} {:>4} {:2}] {}",
                humantime::format_rfc3339_nanos(time),
                record.level(),
                if let Some(s) = record.module_path_static() {
                    s
                } else {
                    ""
                },
                if let Some(v) = record.line() { v } else { 0 },
                std::thread::current().id().as_u64(),
                record.args()
            )
        })
        .init();
}
