use simple_logger::SimpleLogger;

macro_rules! err_log_default {
    ($result:expr, $default:expr) => {
        match $result {
            Ok(val) => val,
            Err(err) => {
                log::error!("{err:?}");
                $default
            }
        }
    };
}

fn main() {
    SimpleLogger::new().init().unwrap();
    log::info!("Start example");

    for input in ["23", "4.2", "42"] {
        let result = input.parse::<u32>();
        let number = err_log_default!(result, 0);
        println!("number: {number}");
    }

    log::info!("End example");
}
