macro_rules! ping {
    ($address: expr) => {
        ping($address, 4);
    };
    ($address: expr, $repeat: expr) => {
        ping($address, $repeat);
    };
}

fn ping(address: &str, mut repeat: u8) {
    while repeat > 0 {
        println!("ping {address}");
        repeat -= 1;
    }
}
fn main() {
    ping("localhost", 1);

    //ping("remote");
    ping!("remote");

    ping!("localhost", 3);
}



