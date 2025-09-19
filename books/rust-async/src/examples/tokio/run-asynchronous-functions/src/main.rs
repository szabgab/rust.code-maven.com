async fn say(text: &str, sec: u64) {
    tokio::time::sleep(tokio::time::Duration::from_secs(sec)).await;
    println!("{text}");
}

async fn call_say() {
    let _ = say("Hello", 2);
    let _ = say("Hi", 1);
}

async fn await_say() {
    say("Hello", 2).await;
    say("Hi", 1).await;
}

async fn spawn_say() {
    tokio::spawn(say("Hello", 2));
    tokio::spawn(say("Hi", 1));
}

async fn wait_say() {
    tokio::spawn(say("Hello", 2));
    tokio::spawn(say("Hi", 1));
    tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;
}

async fn join_say() {
    tokio::join!(
        say("Hello", 2),
        say("Hi", 1),
    );
}

async fn join_set_say() {
    let mut tasks = tokio::task::JoinSet::new();
    tasks.spawn(say("Hello", 2));
    tasks.spawn(say("Hi", 1));
    println!("launched");
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    println!("waited");
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    println!("done");
    tasks.join_all().await;
}

#[tokio::main]
async fn main() {
    let which = get_args();
    let start = std::time::Instant::now();
    match which {
        Case::Call => call_say().await,
        Case::Await => await_say().await,
        Case::Spawn => spawn_say().await,
        Case::Wait => wait_say().await,
        Case::Join => join_say().await,
        Case::JoinSet => join_set_say().await,
    }

    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

#[derive(Debug, PartialEq)]
enum Case {
    Call,
    Await,
    Spawn,
    Wait,
    Join,
    JoinSet,
}
impl std::str::FromStr for Case {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "call" => Ok(Case::Call),
            "await" => Ok(Case::Await),
            "spawn" => Ok(Case::Spawn),
            "wait" => Ok(Case::Wait),
            "join" => Ok(Case::Join),
            "join_set" => Ok(Case::JoinSet),
            _ => Err(format!("Invalid case: {}", input)),
        }
    }
}

fn get_args() -> Case {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <which>", args[0]);
        std::process::exit(1);
    }
    let which = args[1].parse().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    which
}
