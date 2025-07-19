#![allow(clippy::comparison_chain)]

#[derive(Debug)]
enum ExitCode {
    Success,
    Failure(u8),
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let exit = if args.len() == 2 {
        ExitCode::Success
    } else if args.len() < 2 {
        ExitCode::Failure(1)
    } else {
        ExitCode::Failure(2)
    };

    println!("{exit:?}");
    let code = match exit {
        ExitCode::Success => {
            println!("success");
            0
        }
        ExitCode::Failure(err) => {
            println!("Error: {err}");
            err
        }
    };
    println!("{:?}", code);
}
