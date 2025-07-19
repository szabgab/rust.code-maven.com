#[derive(Debug)]
enum ExitCode {
    Success,
    Failure,
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let exit = if args.len() > 1 {
        ExitCode::Success
    } else {
        ExitCode::Failure
    };

    println!("{exit:?}");
}
